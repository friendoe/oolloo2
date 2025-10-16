<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';

	let { model }: { model: any } = $props();

	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 Bytes';
		const k = 1024;
		const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
	}
</script>

<div class="flex items-center justify-between p-4 border-b last:border-b-0">
	<div class="flex items-center gap-4">
		<div class="w-2 h-2 rounded-full bg-green-500"></div>
		<div>
			<p class="font-medium">{model.name}</p>
			<p class="text-sm text-muted-foreground">{model.details.quantization_level}</p>
		</div>
	</div>
	<div class="flex items-center gap-4">
		<Badge variant="outline">{formatBytes(model.size_vram)}</Badge>
	</div>
</div>