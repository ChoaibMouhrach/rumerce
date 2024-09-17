import React from "react";
import { TCollection } from ".";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "~/components/ui/table";
import { Input } from "~/components/ui/input";
import { Switch } from "~/components/ui/switch";

interface CollectionsProps {
  collections: TCollection[];
  setCollections: (collections: TCollection[]) => void;
}

export const Collections: React.FC<CollectionsProps> = ({
  collections,
  setCollections,
}) => {
  const updateCollection = (newCollection: TCollection) => {
    const newCollections = collections.map((collection) => {
      if (
        collection.options.map(({ value }) => value).join("") ===
        newCollection.options.map(({ value }) => value).join("")
      ) {
        return newCollection;
      }

      return collection;
    });

    setCollections(newCollections);
  };

  if (!collections.length) {
    return null;
  }

  return (
    <div className="border rounded-md">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Variant</TableHead>
            <TableHead>Price</TableHead>
            <TableHead className="text-right">Include</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {collections.map((collection, index) => (
            <TableRow key={index}>
              <TableCell className="font-medium">
                {collection.options.map((option) => option.value).join(" / ")}
              </TableCell>
              <TableCell>
                <Input
                  min={1}
                  type="number"
                  className="max-w-[100px]"
                  value={collection.price}
                  onChange={(e) =>
                    updateCollection({
                      ...collection,
                      price: parseFloat(e.target.value),
                    })
                  }
                />
              </TableCell>
              <TableCell className="text-right">
                <Switch
                  checked={collection.checked}
                  onCheckedChange={(checked) =>
                    updateCollection({
                      ...collection,
                      checked,
                    })
                  }
                />
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
};
