import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export interface TCollection {
  price: number;
  checked: boolean;
  options: { key: string; value: string }[];
}

export interface TVariant {
  id: number;
  name: string;
  options: { id: number; name: string }[];
}

export const generateCollections = (
  variants: TVariant[],
  existingCollections: TCollection[]
): TCollection[] => {
  const validVariants = variants
    .filter((variant) => variant.name !== "")
    .map((variant) => ({
      ...variant,
      options: variant.options.filter((option) => option.name !== ""),
    }))
    .filter((variant) => variant.options.length > 0);

  if (validVariants.length === 0) return [];

  const [firstVariant, ...restVariants] = validVariants;
  const restCollections = generateCollections(
    restVariants,
    existingCollections
  );

  if (restCollections.length === 0) {
    return firstVariant.options.map((option) => {
      const options = [{ key: firstVariant.name, value: option.name }];
      const existing = existingCollections.find((collection) =>
        collection.options.every(
          (opt, i) =>
            opt.key === options[i]?.key && opt.value === options[i]?.value
        )
      );
      return {
        price: existing?.price ?? 1,
        checked: existing?.checked ?? false,
        options,
      };
    });
  }

  const collections: TCollection[] = [];
  for (const option of firstVariant.options) {
    for (const restCollection of restCollections) {
      const options = [
        { key: firstVariant.name, value: option.name },
        ...restCollection.options,
      ];
      const isValid = options.every((opt) =>
        validVariants.some(
          (variant) =>
            variant.name === opt.key &&
            variant.options.map(({ name }) => name).includes(opt.value)
        )
      );
      if (isValid) {
        const existing = existingCollections.find((collection) =>
          collection.options.every(
            (opt, i) =>
              opt.key === options[i]?.key && opt.value === options[i]?.value
          )
        );
        collections.push({
          price: existing?.price ?? 1,
          checked: existing?.checked ?? false,
          options,
        });
      }
    }
  }
  return collections;
};
