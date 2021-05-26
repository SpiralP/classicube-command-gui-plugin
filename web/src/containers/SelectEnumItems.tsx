import { Button, MenuItem, Tag } from "@blueprintjs/core";
import { ItemPredicate, ItemRenderer, Select } from "@blueprintjs/select";
import React from "react";

export type EnumItem<T> = {
  label: string;
  value: T;
};

const filterEnumItems: ItemPredicate<{ label: string }> = (
  query,
  { label }
) => {
  return label.toLowerCase().indexOf(query.toLowerCase()) >= 0;
};

const renderEnumItems: ItemRenderer<{ label: string; value: any }> = (
  { label, value },
  { handleClick, modifiers }
) => {
  if (!modifiers.matchesPredicate) {
    return null;
  }
  return (
    <MenuItem
      active={modifiers.active}
      key={value}
      text={label}
      onClick={handleClick}
    />
  );
};

export function SelectEnumItems<T>({
  enumItems,
  text,
  value,
  onItemSelect,
  disabled,
}: {
  enumItems: Array<EnumItem<T>>;
  text: string;
  value: T;
  onItemSelect: (item: EnumItem<T>) => void;
  disabled: boolean;
}) {
  const TypedSelect = Select.ofType<EnumItem<T>>();
  const activeItem = enumItems.find((o) => o.value === value);

  return (
    <>
      <TypedSelect
        disabled={disabled}
        items={enumItems}
        itemPredicate={filterEnumItems}
        itemRenderer={renderEnumItems}
        onItemSelect={(item) => {
          onItemSelect(item);
        }}
        activeItem={activeItem}
      >
        <Button
          text={text}
          rightIcon={<Tag>{activeItem?.label ?? value}</Tag>}
        />
      </TypedSelect>
    </>
  );
}
