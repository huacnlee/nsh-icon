import { open, test } from "nsh/test";

test("labeled icons are read out, and a press swaps the theme icon", async () => {
  const app = await open("./App.tsx");
  await app.waitFor("label:Notifications");
  await app.waitFor("label:Settings");
  await app.press("label:Use dark theme");
  await app.waitFor("label:Use light theme");
});
