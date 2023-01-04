import { useState } from "react";
import { DeleteFilled, EditFilled } from "@ant-design/icons";
import { Button, Input, InputNumber, Modal, Space } from "antd";
import { IProfileCategory } from "./Profile";

interface Props {
  category: IProfileCategory,
  onCategoryChanged: (category: IProfileCategory) => void;
  onCategoryDeleted: () => void;
}

export default function ProfileCategoryEntryActions(props: Props) {
  const [dialogOpen, setDialogOpen] = useState(false);
  const [name, setName] = useState(props.category.name);
  const [priority, setPriority] = useState(props.category.priority);

  return (
    <>
    <Space>
      <Button icon={<EditFilled/>} onClick={() => setDialogOpen(true)}/>
      <Button icon={<DeleteFilled />} danger onClick={props.onCategoryDeleted} />
    </Space>
    <Modal open={dialogOpen} title="Category Properties" onCancel={() => setDialogOpen(false)} okText="Save" onOk={() => {
        const newCategory: IProfileCategory = JSON.parse(JSON.stringify(props.category));
        newCategory.name = name;
        newCategory.priority = priority;
        props.onCategoryChanged(newCategory);
        setDialogOpen(false);
      }} width={300}>
      <Space direction="vertical">
        <Input placeholder="Name" value={name} onChange={(ev) => {setName(ev.target.value);}} />
        <InputNumber placeholder="Priority" value={priority} onChange={value => {setPriority(value ?? 0);}} />
      </Space>
    </Modal>
    </>
  );
}
