<script lang="ts">
  import { onMount } from "svelte";
  import { commands } from "$lib/commands.svelte";
  import { Button } from "$lib/components/ui/button";
  import { Progress } from "$lib/components/ui/progress";
  import { Separator } from "$lib/components/ui/separator";
  import * as Table from "$lib/components/ui/table";
  import { Input } from "$lib/components/ui/input";
  import { ArrowLeft, Search } from "lucide-svelte";
  import type { Model, ModelVersion, PullProgress } from "$lib/types";

  let models = $state<Model[]>([]);
  let selectedModel = $state<Model | null>(null);
  let versions = $state<ModelVersion[]>([]);
  let pulling = $state(false);
  let progress = $state<PullProgress>({ status: "", completed: 0, total: 1, digest: "" });
  let searchQuery = $state("");

  async function searchModels() {
    models = await commands.fetchAllModels(searchQuery);
  }

  onMount(() => {
    searchModels();

    let unlisten: () => void;
    commands.listen<PullProgress>("pull_progress", (event) => {
      progress = event.payload;
      if (progress.status === "success") {
        pulling = false;
      }
    }).then(u => {
        unlisten = u;
    });

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  });

  async function selectModel(model: Model) {
    selectedModel = model;
    versions = await commands.fetchModelDetails(model.url);
  }

  function backToModels() {
    selectedModel = null;
    versions = [];
  }

  async function pullModel(version: string) {
    pulling = true;
    await commands.pullModel(version);
  }

  function handleKeyDown(event: KeyboardEvent, model: Model) {
    if (event.key === 'Enter' || event.key === ' ') {
      selectModel(model);
    }
  }
</script>

<div class="p-4">
  {#if selectedModel}
    <div class="flex items-center gap-2">
      <Button variant="ghost" size="icon" onclick={backToModels}>
        <ArrowLeft class="h-4 w-4" />
      </Button>
      <h1 class="text-2xl font-bold">{selectedModel.name}</h1>
    </div>
    <p class="text-muted-foreground mt-2">{selectedModel.description}</p>
    <div class="mt-4 flex flex-wrap gap-2">
      {#each selectedModel.details as detail}
        <div class="rounded-md bg-muted px-2 py-1 text-xs">{detail}</div>
      {/each}
    </div>
    <div class="mt-4 flex items-center gap-4 text-sm text-muted-foreground">
      <span>{selectedModel.pulls} pulls</span>
      <span>{selectedModel.tags} tags</span>
      <span>Updated {selectedModel.updated}</span>
    </div>

    <h2 class="mt-8 text-xl font-bold">Versions</h2>
    <Separator class="my-4" />

    <Table.Root>
      <Table.Header>
        <Table.Row>
          <Table.Head>Version</Table.Head>
          <Table.Head>Context</Table.Head>
          <Table.Head>Size</Table.Head>
          <Table.Head>Action</Table.Head>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {#each versions as v}
          <Table.Row>
            <Table.Cell>{v.version}</Table.Cell>
            <Table.Cell>{v.context}</Table.Cell>
            <Table.Cell>{v.size}</Table.Cell>
            <Table.Cell>
              {#if pulling}
                <div class="flex items-center gap-2">
                  <Progress value={(progress.completed / progress.total) * 100} />
                  <span>{Math.round((progress.completed / progress.total) * 100)}%</span>
                </div>
              {:else}
                <Button onclick={() => pullModel(v.version)}>Pull</Button>
              {/if}
            </Table.Cell>
          </Table.Row>
        {/each}
      </Table.Body>
    </Table.Root>
  {:else}
    <div class="flex items-center gap-2">
        <Search class="h-4 w-4 text-muted-foreground" />
        <Input bind:value={searchQuery} placeholder="Search models..." oninput={searchModels} />
    </div>
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 mt-4">
      {#each models as model}
        <div
          class="cursor-pointer rounded-lg border p-4"
          role="button"
          tabindex="0"
          onclick={() => selectModel(model)}
          onkeydown={(e) => handleKeyDown(e, model)}
        >
          <h2 class="font-bold">{model.name}</h2>
          <p class="text-muted-foreground mt-2">{model.description}</p>
          <div class="mt-4 flex flex-wrap gap-2">
            {#each model.details as detail}
              <div class="rounded-md bg-muted px-2 py-1 text-xs">{detail}</div>
            {/each}
          </div>
          <div class="mt-4 flex items-center gap-4 text-sm text-muted-foreground">
            <span>{model.pulls} pulls</span>
            <span>{model.tags} tags</span>
            <span>Updated {model.updated}</span>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>