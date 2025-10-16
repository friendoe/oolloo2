<script lang="ts">
	import { onMount, tick } from 'svelte';
	import ChatMessage from './ChatMessage.svelte';
	import { Button } from '$lib/components/ui/button';
	import CustomTextarea from '$lib/components/ui/textarea/CustomTextarea.svelte';
	import { Send } from '@lucide/svelte';
	import { Alert, AlertDescription, AlertTitle } from '$lib/components/ui/alert';

	type Message = {
		role: 'user' | 'assistant';
		content: string;
	};

	type RunningModel = {
		name: string;
		model: string;
	};

	let messages: Message[] = $state([]);
	let inputMessage = $state('');
	let isLoading = $state(false);
	let runningModels: RunningModel[] = $state([]);
	let selectedModel = $state<{ value: string; label: string } | undefined>(undefined);
	let error = $state<string | null>(null);
	let chatContainer: any;

	async function fetchRunningModels() {
		try {
			error = null;
			const response = await fetch('http://localhost:11434/api/ps');
			if (!response.ok) {
				throw new Error('Ollama API is not reachable.');
			}
			const data = await response.json();
			runningModels = data.models || [];

			if (runningModels.length > 0) {
				const newSelectedModel = {
					value: runningModels[0].name,
					label: runningModels[0].name
				};
				selectedModel = newSelectedModel;
				messages = [
					{
						role: 'assistant',
						content: `Hello! I'm ${newSelectedModel.label}. How can I assist you today?`
					}
				];
			} else {
				messages = [
					{
						role: 'assistant',
						content: 'No models are currently running. Please load a model in Ollama to begin.'
					}
				];
			}
		} catch (err: any) {
			console.error('Error fetching running models:', err);
			error = err.message + ' Please ensure Ollama is running.';
			messages = [
				{
					role: 'assistant',
					content: 'Could not connect to Ollama. Please ensure it is running.'
				}
			];
		}
	}

	onMount(() => {
		fetchRunningModels();
	});

	async function scrollToBottom() {
		await tick();
		if (chatContainer) {
			chatContainer.scrollTop = chatContainer.scrollHeight;
		}
	}

	function handleModelChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const modelName = target.value;
		if (!modelName) return;

		const newModel = runningModels.find((m) => m.name === modelName);
		if (newModel) {
			selectedModel = { value: newModel.name, label: newModel.name };
			messages = [{ role: 'assistant', content: `Switched to ${newModel.name}. How can I help?` }];
		}
	}

	async function handleSendMessage(e?: Event) {
		e?.preventDefault();
		if (!inputMessage.trim() || isLoading || !selectedModel) return;

		const userMessage: Message = { role: 'user', content: inputMessage };
		messages = [...messages, userMessage];
		inputMessage = '';
		isLoading = true;
		scrollToBottom();

		try {
			const response = await fetch('http://localhost:11434/api/chat', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					model: selectedModel.value,
					messages: messages,
					stream: true
				})
			});

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}

			const reader = response.body!.getReader();
			const decoder = new TextDecoder();
			let assistantMessage: Message = { role: 'assistant', content: '' };
			messages = [...messages, assistantMessage];

			while (true) {
				const { done, value } = await reader.read();
				if (done) break;

				const chunk = decoder.decode(value, { stream: true });
				const lines = chunk.split('\n');

				for (const line of lines) {
					if (line.trim() === '') continue;
					try {
						const parsed = JSON.parse(line);
						if (parsed.done) break;

						if (parsed.message?.content) {
							assistantMessage.content += parsed.message.content;
							messages = [...messages.slice(0, -1), { ...assistantMessage }];
							scrollToBottom();
						}
					} catch (parseError) {
						console.error('Error parsing JSON:', parseError);
					}
				}
			}
		} catch (err: any) {
			messages = [
				...messages.slice(0, -1),
				{
					role: 'assistant',
					content: 'Sorry, there was an error connecting to the model.'
				}
			];
		} finally {
			isLoading = false;
		}
	}

	const isChatDisabled = $derived(isLoading || runningModels.length === 0 || !selectedModel);
</script>

<div class="flex flex-col h-full">
	<header class="p-2 border-b flex justify-center items-center">
		{#if runningModels.length > 0}
			<div class="w-[280px]">
				<select
					class="w-full p-2 border rounded-md bg-background"
					onchange={handleModelChange}
					value={selectedModel?.value}
				>
					{#each runningModels as model}
						<option value={model.name}>{model.name}</option>
					{/each}
				</select>
			</div>
		{:else}
			<p class="text-lg font-semibold text-muted-foreground">
				{#if error}
					Connection Error
				{:else}
					No Running Models
				{/if}
			</p>
		{/if}
	</header>

	<main bind:this={chatContainer} class="flex-1 p-4 overflow-y-auto space-y-4">
		{#if error}
			<Alert variant="destructive">
				<AlertTitle>Error</AlertTitle>
				<AlertDescription>{error}</AlertDescription>
			</Alert>
		{/if}
		{#each messages as message, i (i)}
			<ChatMessage {message} />
		{/each}
		{#if isLoading && messages[messages.length - 1]?.role === 'user'}
			<ChatMessage message={{ role: 'assistant', content: 'Thinking...' }} />
		{/if}
	</main>

	<footer class="p-2 border-t">
		<form onsubmit={handleSendMessage} class="flex items-center gap-2">
			<CustomTextarea
				bind:value={inputMessage}
				placeholder={isChatDisabled ? 'Please select a running model to start' : 'Type your message...'}
				disabled={isChatDisabled}
				class="min-h-[40px] resize-none"
				onkeydown={(e: KeyboardEvent) => {
					if (e.key === 'Enter' && !e.shiftKey) {
						e.preventDefault();
						handleSendMessage();
					}
				}}
			/>
			<Button type="submit" disabled={isChatDisabled || !inputMessage.trim()}>
				<Send class="h-5 w-5" />
			</Button>
		</form>
	</footer>
</div>