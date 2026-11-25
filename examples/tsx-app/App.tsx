import { mount, signal, Text, View } from "nsh";
import { Icon } from "nsh-icon";
import { MagnifyingGlassSolid, Cog6Tooth } from "nsh-icon-heroicons";
import { Bell, Moon, Search, Settings, Sun } from "nsh-icon-lucide";
import { HouseDuotone, HeartFill } from "nsh-icon-phosphor";

function App() {
  const dark = signal(false);
  return (
    <View class="flex-1 pt-[env(safe-area-inset-top)] pr-[env(safe-area-inset-right)] pb-[env(safe-area-inset-bottom)] pl-[env(safe-area-inset-left)] gap-4 p-6 bg-background">
      <View class="flex-row items-center gap-2 rounded-(--radius) border border-input px-3 py-2">
        <Icon icon={Search} class="text-muted-foreground" />
        <Text class="text-sm text-muted-foreground">Search</Text>
      </View>
      <View class="flex-row items-center gap-3">
        <Icon icon={Bell} size={20} strokeWidth={1.5} label="Notifications" class="text-foreground" />
        <Icon icon={Settings} size={20} label="Settings" class="text-foreground" />
        <View
          testID="theme"
          role="button"
          accessibilityLabel={dark() ? "Use light theme" : "Use dark theme"}
          onPress={() => dark.set(!dark())}
          class="rounded-(--radius) p-2 active:bg-accent"
        >
          <Icon icon={dark() ? Moon : Sun} class="text-primary" />
        </View>
      </View>
      <View class="flex-row items-center gap-3">
        <Icon icon={MagnifyingGlassSolid} size={20} label="Heroicons search" class="text-foreground" />
        <Icon icon={Cog6Tooth} size={20} class="text-foreground" />
        <Icon icon={HouseDuotone} size={20} label="Phosphor home" class="text-primary" />
        <Icon icon={HeartFill} size={20} class="text-destructive" />
      </View>
    </View>
  );
}

mount(App);
