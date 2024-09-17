import { Trash } from "lucide-react";
import React from "react";
import { toast } from "sonner";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";
import { cn } from "~/lib/utils";
import { TCollection } from ".";

export interface TVariant {
  id: number;
  name: string;
  options: { id: number; name: string }[];
}

interface OptionsProps {
  variants: TVariant[];
  setVariants: (variants: TVariant[]) => void;
  collections: TCollection[];
  setCollections: React.Dispatch<React.SetStateAction<TCollection[]>>;
}

export const SubVariants: React.FC<OptionsProps> = ({
  variants,
  setVariants,
  setCollections,
}) => {
  const addVariant = () => {
    const unreadyVariant = variants.find(
      (variant) =>
        variant.name === "" ||
        (variant.options.length === 1 &&
          variant.options.find((option) => option.name === ""))
    );

    if (unreadyVariant) {
      toast.error("Please finish the previous variant first");
      return;
    }

    setVariants([
      ...variants,
      {
        id: Math.random(),
        name: "",
        options: [{ id: Math.random(), name: "" }],
      },
    ]);
  };

  function generateCollections(
    variants: TVariant[],
    existingCollections: TCollection[]
  ): TCollection[] {
    // Filter out variants that have an empty name or all options are empty
    const filteredVariants = variants.filter(
      (variant) =>
        variant.name.trim() !== "" &&
        variant.options.some((option) => option.name.trim() !== "")
    );

    if (filteredVariants.length === 0) {
      return [];
    }

    const [currentVariant, ...remainingVariants] = filteredVariants;
    const subVariants = generateCollections(
      remainingVariants,
      existingCollections
    );

    const validOptions = currentVariant.options.filter(
      (option) => option.name.trim() !== ""
    );

    if (remainingVariants.length === 0) {
      return validOptions.map((option) => {
        const existingCollection = existingCollections.find(
          (col) =>
            col.options.length === 1 && col.options[0].value === option.name
        );
        return {
          options: [{ key: currentVariant.name, value: option.name }],
          price: existingCollection?.price ?? 1, // Preserve price if it exists, otherwise default to 1
          checked: existingCollection?.checked ?? false, // Preserve checked state if it exists
        };
      });
    }

    return validOptions.flatMap((option) =>
      subVariants.map((subVariant) => {
        const newOptions = [
          ...subVariant.options,
          { key: currentVariant.name, value: option.name },
        ];
        const existingCollection = existingCollections.find(
          (col) =>
            col.options.length === newOptions.length &&
            col.options.every(
              (opt, index) => opt.value === newOptions[index].value
            )
        );
        return {
          ...subVariant,
          options: newOptions,
          price: existingCollection?.price ?? 1,
          checked: existingCollection?.checked ?? false,
        };
      })
    );
  }

  const updateVariantName = (id: number, name: string) => {
    const newVariants = variants.map((variant) => {
      if (variant.id === id) {
        return {
          ...variant,
          name,
        };
      }

      return variant;
    });

    setVariants(newVariants);

    setCollections((collections) => {
      return generateCollections(newVariants, collections);
    });
  };

  const updateVariantOption = (
    variantId: number,
    optionId: number,
    name: string
  ) => {
    let isEditingLastOption = false;

    let newVariants = variants.map((variant) => {
      if (variant.id === variantId) {
        let options = variant.options.map((option, index, options) => {
          if (option.id === optionId) {
            isEditingLastOption = index + 1 === options.length;

            return {
              ...option,
              name,
            };
          }

          return option;
        });

        options = options.filter((option, index, options) => {
          return option.name !== "" || index + 1 === options.length;
        });

        return {
          ...variant,
          options,
        };
      }

      return variant;
    });

    if (isEditingLastOption) {
      newVariants = newVariants.map((variant) => {
        if (variant.id === variantId) {
          return {
            ...variant,
            options: [
              ...variant.options,
              {
                id: Math.random(),
                name: "",
              },
            ],
          };
        }

        return variant;
      });
    }

    setVariants(newVariants);

    setCollections((collections) => {
      return generateCollections(newVariants, collections);
    });
  };

  const deleteOption = (variantId: number, optionId: number) => {
    const newVariants = variants.map((variant) => {
      if (variant.id === variantId) {
        return {
          ...variant,
          options: variant.options.filter((option) => option.id !== optionId),
        };
      }

      return variant;
    });

    setVariants(newVariants);

    setCollections((collections) => {
      return generateCollections(newVariants, collections);
    });
  };

  const deleteVariant = (variantId: number) => {
    setVariants(variants.filter((variant) => variant.id !== variantId));
  };

  return (
    <div className="flex flex-col gap-4">
      {variants.map((variant) => (
        <div
          key={variant.id}
          className="p-4 rounded-md border flex flex-col gap-4"
        >
          <div className="flex flex-col gap-3">
            <Label>Name</Label>
            <div className="flex items-center gap-2">
              <Input
                placeholder="Color"
                value={variant.name}
                onChange={(e) => updateVariantName(variant.id, e.target.value)}
              />
              <Button
                className="shrink-0"
                size="icon"
                variant="outline"
                onClick={() => deleteVariant(variant.id)}
              >
                <Trash className="w-3 h-3" />
              </Button>
            </div>
          </div>
          <div className="flex flex-col gap-3">
            <Label>Options</Label>
            <div className="flex flex-col gap-2">
              {variant.options.map((option, index, options) => (
                <div className="flex items-center gap-2" key={option.id}>
                  <Input
                    className={cn(
                      option.name === "" && index + 1 !== options.length
                        ? "border-destructive"
                        : ""
                    )}
                    placeholder="Red"
                    value={option.name}
                    onChange={(e) =>
                      updateVariantOption(variant.id, option.id, e.target.value)
                    }
                  />
                  <Button
                    className="shrink-0"
                    onClick={() => deleteOption(variant.id, option.id)}
                    disabled={index + 1 === options.length}
                    size="icon"
                    variant="outline"
                  >
                    <Trash className="w-3 h-3" />
                  </Button>
                </div>
              ))}
            </div>
          </div>
        </div>
      ))}
      <Button onClick={addVariant} variant="outline">
        Add variant
      </Button>
    </div>
  );
};
