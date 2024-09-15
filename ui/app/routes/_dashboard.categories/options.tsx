import { Settings } from "lucide-react";
import { Button } from "~/components/ui/button";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "~/components/ui/sheet";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs";
import { Update } from "./update";
import { Delete } from "./delete";
import React, { useState } from "react";
import { Category } from "~/validations/category";

interface OptionsProps {
  category: Category;
}

export const Options: React.FC<OptionsProps> = ({ category }) => {
  const [open, setOpen] = useState(false);

  return (
    <Sheet open={open} onOpenChange={setOpen}>
      <SheetTrigger asChild>
        <Button size="sm" variant="ghost">
          <Settings className="w-4 h-4" />
        </Button>
      </SheetTrigger>
      <SheetContent className="flex flex-col gap-6">
        <SheetHeader>
          <SheetTitle>Category settings</SheetTitle>
          <SheetDescription>
            You can manage this category from here.
          </SheetDescription>
        </SheetHeader>

        <Tabs defaultValue="update">
          <TabsList>
            <TabsTrigger value="update">Update</TabsTrigger>
            <TabsTrigger value="delete">Delete</TabsTrigger>
          </TabsList>
          <TabsContent value="update">
            <Update category={category} />
          </TabsContent>
          <TabsContent value="delete">
            <Delete id={category.id} close={() => setOpen(false)} />
          </TabsContent>
        </Tabs>
      </SheetContent>
    </Sheet>
  );
};
