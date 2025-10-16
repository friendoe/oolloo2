<script lang="ts">
	import { cn } from '$lib/utils.js';

	let { class: className = undefined, ...rest }: { class?: string } = $props();
</script>

<h5
	class={cn('mb-1 font-medium leading-none tracking-tight', className)}
	{...rest}
>
	<slot />
</h5>