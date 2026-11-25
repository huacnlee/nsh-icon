#!/usr/bin/env bun
// Sync an icon set's pinned release (sets/<set>/set.json) into sets/<set>/:
// every icon's SVG byte for byte, flattened into icons/<name>.svg under the
// names Iconify gives them (a style is a suffix: `home-solid`,
// `house-bold`), its old names (aliases.txt) and its license (LICENSE).
//
//   bun script/sync.ts                  every set
//   bun script/sync.ts lucide tabler    these sets
//   --check                             verify without writing
//   --archive <file>                    a local copy of the one set's archive
//
// Then regenerate each set's Rust and TSX:
//   NSH_ICON_UPDATE=1 cargo test -p nsh-icon-codegen
import { readdir, rm, mkdir } from "node:fs/promises";
import { resolve } from "node:path";
import { parseArgs } from "node:util";

type Entries = Map<string, Uint8Array>;

interface SetSource {
  /** The icon an archive file holds (its path without the archive's top directory), or null. */
  icon(path: string): string | null;
  /** The license's path in the archive. */
  license: string;
  /** Old names: `<old> <current>` lines. */
  aliases?(entries: Entries): string[];
}

const svg = (path: string, dir: string, suffix = "") => {
  if (!path.startsWith(`${dir}/`) || !path.endsWith(".svg")) return null;
  const name = path.slice(dir.length + 1, -".svg".length);
  return name.includes("/") ? null : `${name}${suffix}`;
};

const text = (bytes: Uint8Array) => new TextDecoder().decode(bytes);

const SOURCES: Record<string, SetSource> = {
  lucide: {
    icon: (path) => svg(path, "icons"),
    license: "LICENSE",
    aliases(entries) {
      const lines: string[] = [];
      for (const [path, bytes] of entries) {
        const icon = path.match(/^icons\/([^/]+)\.json$/)?.[1];
        if (!icon) continue;
        for (const alias of JSON.parse(text(bytes)).aliases ?? []) {
          lines.push(`${typeof alias === "string" ? alias : alias.name} ${icon}`);
        }
      }
      return lines;
    },
  },
  heroicons: {
    icon: (path) =>
      svg(path, "optimized/24/outline") ??
      svg(path, "optimized/24/solid", "-solid") ??
      svg(path, "optimized/20/solid", "-20-solid") ??
      svg(path, "optimized/16/solid", "-16-solid"),
    license: "LICENSE",
  },
  tabler: {
    icon: (path) => svg(path, "icons/outline") ?? svg(path, "icons/filled", "-filled"),
    license: "LICENSE",
    aliases(entries) {
      const aliases = JSON.parse(text(entries.get("aliases.json")!));
      return [
        ...Object.entries(aliases.outline ?? {}).map(([old, icon]) => `${old} ${icon}`),
        ...Object.entries(aliases.filled ?? {}).map(([old, icon]) => `${old}-filled ${icon}-filled`),
      ];
    },
  },
  phosphor: {
    // assets/<weight>/<name>.svg, whose name has the weight already
    // (`house-bold`), but regular's (`house`).
    icon: (path) => {
      const parts = path.split("/");
      return parts.length === 3 && parts[0] === "assets" && parts[2].endsWith(".svg")
        ? parts[2].slice(0, -".svg".length)
        : null;
    },
    license: "LICENSE",
  },
  remix: {
    icon: (path) => {
      const parts = path.split("/");
      return parts.length === 3 && parts[0] === "icons" && parts[2].endsWith(".svg") ? parts[2].slice(0, -4) : null;
    },
    license: "License",
  },
};

async function sync(set: string, check: boolean, archive?: string) {
  const source = SOURCES[set];
  if (!source) throw new Error(`no icon set ${set}: ${Object.keys(SOURCES).join(", ")}`);
  const root = resolve(import.meta.dir, "../sets", set);
  const manifest: { version: string; archive_url: string; sha256: string; icon_count: number } =
    await Bun.file(resolve(root, "set.json")).json();

  let data: Uint8Array;
  if (archive) {
    data = await Bun.file(archive).bytes();
  } else {
    const response = await fetch(manifest.archive_url, { signal: AbortSignal.timeout(120_000) });
    if (!response.ok) throw new Error(`${set}: download failed: HTTP ${response.status}`);
    data = new Uint8Array(await response.arrayBuffer());
  }
  if (new Bun.CryptoHasher("sha256").update(data).digest("hex") !== manifest.sha256) {
    throw new Error(`${set}: archive checksum mismatch`);
  }

  // Paths without the archive's top directory (`lucide-1.43.0/`, `package/`).
  const entries: Entries = new Map();
  for (const [name, file] of await new Bun.Archive(data).files()) {
    const path = name.split("/").slice(1).join("/");
    if (path) entries.set(path, new Uint8Array(await file.arrayBuffer()));
  }

  const files = new Map<string, Uint8Array>();
  for (const [path, bytes] of entries) {
    const icon = source.icon(path);
    if (icon === null) continue;
    if (files.has(`icons/${icon}.svg`)) throw new Error(`${set}: two files are the icon ${icon}`);
    files.set(`icons/${icon}.svg`, bytes);
  }
  const count = files.size;
  if (count !== manifest.icon_count) {
    throw new Error(`${set}: ${count} icons, set.json says ${manifest.icon_count}`);
  }
  const license = entries.get(source.license);
  if (!license) throw new Error(`${set}: no ${source.license} in the archive`);
  files.set("LICENSE", license);
  const aliases = (source.aliases?.(entries) ?? []).sort();
  if (aliases.length) {
    files.set(
      "aliases.txt",
      new TextEncoder().encode(
        `# Old ${set} ${manifest.version} names: <alias> <icon>. Generated by script/sync.ts.\n` +
          aliases.map((line) => `${line}\n`).join(""),
      ),
    );
  }

  let changed = 0;
  if (!check) await mkdir(resolve(root, "icons"), { recursive: true });
  for (const [name, content] of files) {
    const target = Bun.file(resolve(root, name));
    if (!(await target.exists()) || !Buffer.from(content).equals(Buffer.from(await target.arrayBuffer()))) {
      changed++;
      if (!check) await Bun.write(target, content);
    }
  }
  // Icons a release removed or renamed leave the set.
  const present = await readdir(resolve(root, "icons")).catch(() => [] as string[]);
  for (const entry of present) {
    if (!files.has(`icons/${entry}`)) {
      changed++;
      if (!check) await rm(resolve(root, "icons", entry));
    }
  }
  if (check && changed) {
    throw new Error(`${set}: ${changed} files missing or out of date; run bun script/sync.ts ${set}`);
  }
  console.log(
    `${set} ${manifest.version}: ${count} icons, ${aliases.length} aliases; ${changed} files ${check ? "differ" : "updated"}`,
  );
}

async function main() {
  const { values, positionals } = parseArgs({
    args: Bun.argv.slice(2),
    options: {
      check: { type: "boolean", default: false },
      archive: { type: "string" },
      help: { type: "boolean", short: "h" },
    },
    strict: true,
    allowPositionals: true,
  });
  if (values.help) {
    console.log("Usage: bun script/sync.ts [--check] [--archive archive] [set...]");
    return;
  }
  const sets = positionals.length ? positionals : Object.keys(SOURCES);
  if (values.archive && sets.length !== 1) throw new Error("--archive needs exactly one set");
  for (const set of sets) await sync(set, values.check, values.archive);
}

main().catch((error) => {
  console.error(error instanceof Error ? error.message : String(error));
  process.exitCode = 1;
});
