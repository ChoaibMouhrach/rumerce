import React, { useEffect, useState } from "react";
import { Collections } from "./collections";
import { SubVariants, TVariant } from "./sub-variants";

export interface TCollection {
  price: number;
  checked: boolean;
  options: { key: string; value: string }[];
}

interface VariantsProps {
  value: TCollection[];
  onValueChange: (value: TCollection[]) => void;
}

export const Variants: React.FC<VariantsProps> = ({ value, onValueChange }) => {
  const [variants, setVariants] = useState<TVariant[]>([]);
  const [collections, setCollections] = useState<TCollection[]>(value);

  useEffect(() => {
    onValueChange(collections);
  }, [collections, onValueChange]);

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
