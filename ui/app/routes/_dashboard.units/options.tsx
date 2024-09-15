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
import { Unit } from "~/validations/unit";

interface OptionsProps {
  unit: Unit;
}

export const Options: React.FC<OptionsProps> = ({ unit: unit }) => {
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
          <SheetTitle>Unit settings</SheetTitle>
          <SheetDescription>
            You can manage this unit from here.
          </SheetDescription>
        </SheetHeader>

        <Tabs defaultValue="update">
          <TabsList>
            <TabsTrigger value="update">Update</TabsTrigger>
            <TabsTrigger value="delete">Delete</TabsTrigger>
          </TabsList>
          <TabsContent value="update">
            <Update unit={unit} />
          </TabsContent>
          <TabsContent value="delete">
            <Delete id={unit.id} close={() => setOpen(false)} />
          </TabsContent>
        </Tabs>
      </SheetContent>
    </Sheet>
  );
};
