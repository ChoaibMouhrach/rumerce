import React, { useEffect, useState } from "react";
import * as dropzone from "react-dropzone";
import { AspectRatio } from "./ui/aspect-ratio";
import { X } from "lucide-react";
import { Button } from "./ui/button";

interface DropZoneProps {
  onValueChange: (files: File[]) => void;
  value: File[];
}

export const DropZone: React.FC<DropZoneProps> = ({ onValueChange, value }) => {
  const [acceptedFiles, setAcceptedFiles] = useState<File[]>([]);

  const { getRootProps, getInputProps, isDragActive } = dropzone.useDropzone({
    onDrop: (newAcceptedFiles) => {
      const newFiles = [...acceptedFiles, ...newAcceptedFiles];
      onValueChange(newFiles);
      setAcceptedFiles(newFiles);
    },
    accept: {
      "image/*": [],
    },
    multiple: true,
  });

  const onDelete = (targetIndex: number) => {
    setAcceptedFiles(
      acceptedFiles.filter((_, index) => {
        return index !== targetIndex;
      })
    );
  };

  useEffect(() => setAcceptedFiles(value), [value]);

  return (
    <div className="flex flex-col gap-2">
      <div
        {...getRootProps()}
        className="relative h-32 overflow-hidden rounded-md"
      >
        <input {...getInputProps()} />
        <div className="absolute top-0 text-sm md:text-base left-0 w-full h-full bg-muted rounded-md border flex items-center justify-center text-muted-foreground cursor-pointer hover:border-primary duration-200">
          {isDragActive ? (
            <p>Drop the files here ...</p>
          ) : (
            <p>Drag {"'n'"} drop some files here, or click to select files</p>
          )}
        </div>
      </div>
      <div className="grid grid-cols-3 gap-2">
        {acceptedFiles.map((file, index) => (
          <AspectRatio
            key={index}
            className="relative border rounded-md overflow-hidden bg-muted group"
            ratio={1}
          >
            <img
              src={URL.createObjectURL(file)}
              alt={file.name}
              className="absolute top-0 left-0 w-full h-full object-contain"
            />

            <div className="opacity-0 group-hover:opacity-100 duration-200 pointer-events-none group-hover:pointer-events-auto absolute top-0 left-0 w-full h-full bg-black/60 flex items-center justify-center text-white">
              <Button
                variant="ghost"
                size="icon"
                onClick={() => onDelete(index)}
              >
                <X className="w-6 h-6" />
              </Button>
            </div>
          </AspectRatio>
        ))}
      </div>
    </div>
  );
};
