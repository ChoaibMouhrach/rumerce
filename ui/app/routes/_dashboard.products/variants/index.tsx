import React, { useEffect, useState } from "react";
import { Collections } from "./collections";
import { generateCollections, TCollection, TVariant } from "~/lib/utils";
import { SubVariants } from "./sub-variants";

interface VariantsProps {
  value: TCollection[];
  onValueChange: (value: TCollection[]) => void;
}

export const Variants: React.FC<VariantsProps> = ({ value, onValueChange }) => {
  const [variants, setVariants] = useState<TVariant[]>([]);
  const [collections, setCollections] = useState<TCollection[]>([]);

  console.log(variants);

  useEffect(() => {
    onValueChange(collections);
  }, [collections, onValueChange]);

  useEffect(() => {
    const vs: Record<string, string[]> = {};

    for (const variant of value) {
      for (const option of variant.options) {
        if (!vs[option.key]) {
          vs[option.key] = [option.value];
          continue;
        }

        if (vs[option.key].find((value) => option.value === value)) {
          continue;
        }

        vs[option.key].push(option.value);
      }
    }

    const variants = Object.entries(vs).map(([name, options]) => {
      return {
        id: Math.random(),
        name: name,
        options: [
          ...options.map((option) => ({
            id: Math.random(),
            name: option,
          })),
          {
            id: Math.random(),
            name: "",
          },
        ],
      };
    });

    const collections = generateCollections(variants, value);

    setVariants(variants);
    setCollections(collections);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="flex flex-col gap-4">
      <SubVariants
        variants={variants}
        setVariants={setVariants}
        collections={collections}
        setCollections={setCollections}
      />
      <Collections collections={collections} setCollections={setCollections} />
    </div>
  );
};
