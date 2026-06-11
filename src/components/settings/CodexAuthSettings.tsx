import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { KeyRound, Image } from "lucide-react";
import { toast } from "sonner";
import type { SettingsFormState } from "@/hooks/useSettings";
import { ToggleRow } from "@/components/ui/toggle-row";
import { settingsApi, type CopilotOptimizerConfig } from "@/lib/api/settings";

interface CodexAuthSettingsProps {
  settings: SettingsFormState;
  onChange: (updates: Partial<SettingsFormState>) => void;
}

export function CodexAuthSettings({
  settings,
  onChange,
}: CodexAuthSettingsProps) {
  const { t } = useTranslation();
  const [copilotOptimizerConfig, setCopilotOptimizerConfig] = useState<CopilotOptimizerConfig>({
    enabled: true,
    requestClassification: true,
    toolResultMerging: true,
    compactDetection: true,
    deterministicRequestId: true,
    subagentDetection: true,
    warmupDowngrade: true,
    warmupModel: "gpt-5-mini",
    stripThinking: true,
    disableImageGeneration: true,
  });

  useEffect(() => {
    settingsApi
      .getCopilotOptimizerConfig()
      .then(setCopilotOptimizerConfig)
      .catch((e) => console.error("Failed to load copilot optimizer config:", e));
  }, []);

  const handleCopilotOptimizerChange = async (updates: Partial<CopilotOptimizerConfig>) => {
    const newConfig = { ...copilotOptimizerConfig, ...updates };
    setCopilotOptimizerConfig(newConfig);
    try {
      await settingsApi.setCopilotOptimizerConfig(newConfig);
    } catch (e) {
      console.error("Failed to save copilot optimizer config:", e);
      toast.error(String(e));
      setCopilotOptimizerConfig(copilotOptimizerConfig);
    }
  };

  return (
    <section className="space-y-4">
      <div className="flex items-center gap-2 pb-2 border-b border-border/40">
        <KeyRound className="h-4 w-4 text-primary" />
        <h3 className="text-sm font-medium">{t("settings.codexAuth")}</h3>
      </div>

      <ToggleRow
        icon={<KeyRound className="h-4 w-4 text-emerald-500" />}
        title={t("settings.preserveCodexOfficialAuthOnSwitch")}
        description={t("settings.preserveCodexOfficialAuthOnSwitchDescription")}
        checked={settings.preserveCodexOfficialAuthOnSwitch ?? false}
        onCheckedChange={(value) =>
          onChange({ preserveCodexOfficialAuthOnSwitch: value })
        }
      />

      <ToggleRow
        icon={<Image className="h-4 w-4 text-blue-500" />}
        title={t("settings.disableImageGeneration")}
        description={t("settings.disableImageGenerationDescription")}
        checked={copilotOptimizerConfig.disableImageGeneration ?? false}
        onCheckedChange={(value) =>
          handleCopilotOptimizerChange({ disableImageGeneration: value })
        }
      />
    </section>
  );
}
