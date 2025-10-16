import Root from './alert.svelte';
import { tv, type VariantProps } from 'tailwind-variants';
import AlertTitle from './alert-title.svelte';
import AlertDescription from './alert-description.svelte';

const alertVariants = tv({
	base: 'relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground',
	variants: {
		variant: {
			default: 'bg-background text-foreground',
			destructive:
				'border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive'
		}
	},
	defaultVariants: {
		variant: 'default'
	}
});

type Variant = VariantProps<typeof alertVariants>['variant'];
type Props = {
	variant?: Variant;
};

export {
	Root,
	AlertTitle,
	AlertDescription,
	alertVariants,
	type Props,
	type Variant,
	//
	Root as Alert
};