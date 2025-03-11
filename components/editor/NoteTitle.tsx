import React, { useState, useEffect, KeyboardEvent } from 'react';
import { cn } from '@/lib/utils'; 

interface DocumentTitleProps extends React.HTMLAttributes<HTMLDivElement> {
  placeholder?: string;
  value?: string;
  onEdit?: (value: string) => void;
  onExit?: () => void; 
}

const NoteTitle = React.forwardRef<HTMLDivElement, DocumentTitleProps>(
  ({ className, placeholder = 'Untitled', value = '', onEdit, onExit, ...props }, ref) => {
    const [isEmpty, setIsEmpty] = useState(!value);
    const divRef = ref as React.RefObject<HTMLDivElement>;

    useEffect(() => {
      if (divRef.current && divRef.current.innerHTML !== value && value !== undefined) {
        divRef.current.innerHTML = value;
      }
      setIsEmpty(!value);
    }, [value, divRef]);

    const handleInput = (e: React.FormEvent<HTMLDivElement>) => {
      const content = e.currentTarget.innerHTML;
      const isContentEmpty = content === '';
      
      setIsEmpty(isContentEmpty);
      
      if (onEdit) {
        onEdit(isContentEmpty ? '' : content);
      }
    };

    const handleKeyDown = (e: KeyboardEvent<HTMLDivElement>) => {
      const textContent = divRef.current?.textContent || '';
      if (e.key === ' ' && !textContent.trim()) {
        e.preventDefault();
        return;
      }

      if ((e.key === 'Enter' || e.key === 'ArrowDown') && onExit) {
        e.preventDefault();
        onExit();
      }
    };

    const handleFocus = () => {
      if (isEmpty && divRef.current) {
        divRef.current.innerHTML = '';
      }
    };

    return (
      <div
        ref={divRef}
        className={cn(
          className,
          isEmpty ? 'before:content-[attr(data-placeholder)] before:text-gray-400 before:absolute' : ''
        )}
        contentEditable
        suppressContentEditableWarning
        data-placeholder={placeholder}
        onInput={handleInput}
        onKeyDown={handleKeyDown}
        onFocus={handleFocus}
        {...props}
      />
    );
  }
);

NoteTitle.displayName = "NoteTitle"

export default NoteTitle;