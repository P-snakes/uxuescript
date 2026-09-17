import { computed, ref } from "vue";
import { defineStore } from "pinia";
import {
  commands,
  type LLMProvider,
  type OptionsConfig,
} from "@/services/cmds";

const cloneProviders = (providers: Record<string, LLMProvider>) =>
  Object.fromEntries(
    Object.entries(providers).map(([id, provider]) => [
      id,
      { ...provider, models: [...provider.models] },
    ]),
  );

const providerMap = (items: LLMProvider[]) =>
  Object.fromEntries(
    items.map((provider) => [
      provider.is_custom ? provider.name : provider.name.toLowerCase(),
      { ...provider, models: [...provider.models] },
    ]),
  );

export const useConfigStore = defineStore("config", () => {
  const providers = ref<Record<string, LLMProvider>>({});
  const activeId = ref("");
  const options = ref<OptionsConfig>();
  const busy = ref(false);
  const error = ref("");
  const initialized = ref(false);
  const initialProviders = ref<Record<string, LLMProvider>>({});

  const entries = computed(() => Object.entries(providers.value));
  const selectedProvider = computed(() => providers.value[activeId.value]);

  const run = async (operation: () => Promise<void>) => {
    if (busy.value) return false;
    busy.value = true;
    error.value = "";
    try {
      await operation();
      return true;
    } catch (cause) {
      error.value = cause instanceof Error ? cause.message : String(cause);
      return false;
    } finally {
      busy.value = false;
    }
  };

  const initialize = async () => {
    if (initialized.value) return true;
    return run(async () => {
      const [items, current, courseOptions] = await Promise.all([
        commands.providers(),
        commands.currentProvider(),
        commands.options(),
      ]);
      providers.value = providerMap(items);
      initialProviders.value = cloneProviders(providers.value);
      activeId.value = current;
      options.value = { ...courseOptions };
      initialized.value = true;
    });
  };

  const selectProvider = (index: number) => {
    const entry = entries.value[index];
    if (entry) activeId.value = entry[0];
  };

  const selectModel = (index: number) => {
    const provider = selectedProvider.value;
    if (!provider || index < 0 || index >= provider.models.length) return;
    provider.chosen_model = index;
  };

  const createProvider = (name: string) => {
    if (name in providers.value) throw new Error("Provider ID already exists");
    if (entries.value.some(([, provider]) => provider.name === name))
      throw new Error("Provider name already exists");
    providers.value[name] = {
      name,
      is_custom: true,
      protocol: "OpenAIChatCompletions",
      base_url: "",
      api_key: null,
      models: [],
      chosen_model: null,
    };
    activeId.value = name;
  };

  const renameProvider = (index: number, name: string) => {
    const entry = entries.value[index];
    if (!entry) return;
    const [id, provider] = entry;
    if (!provider.is_custom) return;
    if (name !== id && name in providers.value)
      throw new Error("Provider ID already exists");
    if (
      entries.value.some(
        ([otherId, otherProvider]) =>
          otherId !== id && otherProvider.name === name,
      )
    ) {
      throw new Error("Provider name already exists");
    }
    if (name === id) return;
    const next = cloneProviders(providers.value);
    delete next[id];
    next[name] = { ...provider, name };
    providers.value = next;
    if (activeId.value === id) activeId.value = name;
  };

  const deleteProvider = (index: number) => {
    const entry = entries.value[index];
    if (!entry || !entry[1].is_custom) return;
    const [id] = entry;
    const remaining = entries.value.filter(([key]) => key !== id);
    if (remaining.length === 0)
      throw new Error("The only Provider cannot be deleted");
    const next = cloneProviders(providers.value);
    delete next[id];
    providers.value = next;
    if (activeId.value === id) activeId.value = remaining[0][0];
  };

  const saveModel = (index: number | null, name: string) => {
    const provider = selectedProvider.value;
    if (!provider?.is_custom) return;
    if (
      provider.models.some(
        (model, position) => position !== index && model === name,
      )
    ) {
      throw new Error("Model name already exists");
    }
    const position = index ?? provider.models.length;
    provider.models[position] = name;
    provider.chosen_model = position;
  };

  const deleteModel = (index: number) => {
    const provider = selectedProvider.value;
    if (!provider?.is_custom || index < 0 || index >= provider.models.length)
      return;
    provider.models.splice(index, 1);
    const chosen = provider.chosen_model;
    provider.chosen_model =
      chosen === null || provider.models.length === 0
        ? null
        : chosen > index
          ? chosen - 1
          : Math.min(chosen, provider.models.length - 1);
  };

  const save = async () =>
    run(async () => {
      const active = selectedProvider.value;
      if (!active) throw new Error("Select a Provider before saving");

      for (const provider of Object.values(providers.value)) {
        if (provider.is_custom) await commands.upsertProvider(provider);
      }
      await commands.switchProvider(activeId.value);
      if (active.is_custom) await commands.upsertProvider(active);
      else await commands.setKey(active.api_key ?? "");
      if (active.chosen_model !== null)
        await commands.switchModel(active.models[active.chosen_model]);

      for (const [id, provider] of Object.entries(initialProviders.value)) {
        if (provider.is_custom && !(id in providers.value))
          await commands.removeProvider(id);
      }
      if (options.value) await commands.setOptions(options.value);
      await commands.saveConfig();
      initialProviders.value = cloneProviders(providers.value);
    });

  return {
    providers,
    activeId,
    options,
    busy,
    error,
    entries,
    selectedProvider,
    initialize,
    selectProvider,
    selectModel,
    createProvider,
    renameProvider,
    deleteProvider,
    saveModel,
    deleteModel,
    save,
  };
});
