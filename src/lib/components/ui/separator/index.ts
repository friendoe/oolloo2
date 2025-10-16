import Root from './separator.svelte';
import { tv, type VariantProps } from 'tailwind-variants';

const separatorVariants = tv({
	base: 'shrink-0 bg-border',
	variants: {
		orientation: {
			horizontal: 'h-[1px] w-full',
			vertical: 'h-full w-[1px]'
		}
	},
	defaultVariants: {
		orientation: 'horizontal'
	}
});

type Orientation = VariantProps<typeof separatorVariants>['orientation'];
type Props = {
	orientation?: Orientation;
};

export {
	Root,
	separatorVariants,
	type Props,
	type Orientation,
	//
	Root as Separator
};