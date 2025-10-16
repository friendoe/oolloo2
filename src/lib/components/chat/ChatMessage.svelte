<script lang="ts">
	import { cn } from '$lib/utils';
	import { Bot, User } from 'lucide-svelte';

	type Message = {
		role: 'user' | 'assistant';
		content: string;
	};

	let { message }: { message: Message } = $props();
</script>

<div
	class={cn(
		'flex items-start gap-4',
		message.role === 'user' ? 'justify-end' : 'justify-start'
	)}
>
	{#if message.role === 'assistant'}
		<div class="flex-shrink-0 h-8 w-8 rounded-full bg-primary flex items-center justify-center text-primary-foreground">
			<Bot class="h-5 w-5" />
		</div>
	{/if}

	<div
		class={cn(
			'p-3 rounded-lg max-w-[75%]',
			message.role === 'user'
				? 'bg-primary text-primary-foreground'
				: 'bg-muted text-muted-foreground'
		)}
	>
		<p class="whitespace-pre-wrap">{message.content}</p>
	</div>

	{#if message.role === 'user'}
		<div class="flex-shrink-0 h-8 w-8 rounded-full bg-muted flex items-center justify-center text-muted-foreground">
			<User class="h-5 w-5" />
		</div>
	{/if}
</div>