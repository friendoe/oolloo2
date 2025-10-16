<script lang="ts">
	import { cn } from '$lib/utils.js';

	let { class: className = undefined, ...rest }: { class?: string } = $props();
</script>

<div
	class={cn('text-sm [&_p]:leading-relaxed', className)}
	{...rest}
>
	<slot />
</div>