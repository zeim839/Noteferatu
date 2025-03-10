import React, { useRef, useEffect, useState } from 'react';

interface EditableDivProps {
  placeholder?: string;
  className?: string;
  value: string;
  onChange: (value: string) => void;
}

const EditableDiv: React.FC<EditableDivProps> = ({
  placeholder = 'Untitled',
  className = '',
  value,
  onChange
}) => {
  const divRef = useRef<HTMLDivElement>(null);
  const [isEmpty, setIsEmpty] = useState(!value);

  useEffect(() => {
    if (divRef.current && divRef.current.innerHTML !== value) {
      divRef.current.innerHTML = value;
    }
  }, [value]);

  const handleInput = (e: React.FormEvent<HTMLDivElement>) => {
    const content = e.currentTarget.innerHTML;
    const isContentEmpty = content === '' || content === '<br>' || content === '<div><br></div>';
    
    setIsEmpty(isContentEmpty);
    onChange(isContentEmpty ? '' : content);
  };

  const handleFocus = () => {
    if (isEmpty && divRef.current) {
      divRef.current.innerHTML = '';
    }
  };

  const handleBlur = () => {
    if (divRef.current && (divRef.current.innerHTML === '' || divRef.current.innerHTML === '<br>')) {
      setIsEmpty(true);
      divRef.current.innerHTML = '';
      onChange('');
    }
  };

  return (
    <div
      ref={divRef}
      className={`${className} ${isEmpty ? 'before:content-[attr(data-placeholder)] before:text-gray-400 before:absolute' : ''}`}
      contentEditable
      suppressContentEditableWarning={true}
      data-placeholder={placeholder}
      onInput={handleInput}
      onFocus={handleFocus}
      onBlur={handleBlur}
    />
  );
};

export default EditableDiv;