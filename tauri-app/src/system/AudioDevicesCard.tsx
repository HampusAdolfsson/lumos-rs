import React, { useState, useEffect, useContext, useMemo } from 'react';
import { Button, Card, Input, Table } from 'antd';
import { ColumnsType } from 'antd/es/table';
import {  DeleteFilled, HolderOutlined, PlusOutlined } from '@ant-design/icons';
import { AudioDevicesService } from './AudioDevicesService';
import { DndContext, DragEndEvent, UniqueIdentifier } from '@dnd-kit/core';
import { CSS } from '@dnd-kit/utilities';
import { arrayMove, SortableContext, useSortable, verticalListSortingStrategy } from '@dnd-kit/sortable';
import { restrictToVerticalAxis } from '@dnd-kit/modifiers';
import { SyntheticListenerMap } from '@dnd-kit/core/dist/hooks/utilities';

interface RowContextProps {
  setActivatorNodeRef?: (element: HTMLElement | null) => void;
  listeners?: SyntheticListenerMap;
}

const RowContext = React.createContext<RowContextProps>({});

const DragHandle: React.FC = () => {
  const { setActivatorNodeRef, listeners } = useContext(RowContext);
  return (
    <Button
      type="text"
      size="small"
      icon={<HolderOutlined />}
      style={{ cursor: 'move' }}
      ref={setActivatorNodeRef}
      {...listeners}
    />
  );
};

interface RowProps extends React.HTMLProps<HTMLTableRowElement> {
  "data-row-key": UniqueIdentifier,
}

const Row: React.FC<RowProps> = (props) => {
  const {
      attributes,
      listeners,
      setNodeRef,
      setActivatorNodeRef,
      transform,
      transition,
      isDragging,
    } = useSortable({ id: props["data-row-key"] });

  const style: React.CSSProperties = {
    ...props.style,
    transform: CSS.Translate.toString(transform),
    transition,
    ...(isDragging ? { position: 'relative', zIndex: 9999 } : {}),
  };

  const contextValue = useMemo<RowContextProps>(
    () => ({ setActivatorNodeRef, listeners }),
    [setActivatorNodeRef, listeners],
  );

  return (
    <RowContext.Provider value={contextValue}>
      <tr {...props} ref={setNodeRef} style={style} {...attributes} />
    </RowContext.Provider>
  );
}

export function AudioDevicesCard() {
  const [devices, setDevices] = useState([] as Array<string>);

  useEffect(() => {
    const subscription = AudioDevicesService.Instance().then(service => service.audioDevices.subscribe(devs => setDevices(devs)));
    return () => {
      subscription.then(sub => sub.unsubscribe());
    };
  });

  const columns: ColumnsType<{ name: string }> = [
    { key: 'sort', align: 'center', width: 80, render: () => <DragHandle /> },
    {
      key: "name",
      render: (_, device) => (
        <span>{device.name}</span>
      ),
    },
    {
      key: "action",
      align: "right",
      render: (_, device) => (
        <Button danger
          icon={<DeleteFilled />}
          onClick={async() => {
            const newDevs = devices.filter(name => name !== device.name);
            (await AudioDevicesService.Instance()).setAudioDevices(newDevs, true);
            setDevices(newDevs);
          }}/>
      )
    }
  ];

  const [showingInputField, setShowingInputField] = useState(false);
  const [inputFieldValue, setInputFieldValue] = useState("");
  const inputField =
    <Input value={inputFieldValue}
      onBlur={() => {
        setShowingInputField(false);
        setInputFieldValue("");
      }}
      onKeyDown={async(ev) => {
        if (ev.key === "Enter") {
          const newDevs = JSON.parse(JSON.stringify(devices));
          newDevs.push(inputFieldValue);
          (await AudioDevicesService.Instance()).setAudioDevices(newDevs, true);
          setDevices(newDevs);
          setInputFieldValue("");
          setShowingInputField(false);
        }
      }}
      onChange={e => setInputFieldValue(e.target.value)}
      placeholder="Enter a (partial) device name"
      autoFocus={true} />

  const onDragEnd = async({ active, over }: DragEndEvent) => {
    if (active.id !== over?.id) {
        const activeIndex = devices.findIndex(device => device === active?.id);
        const overIndex = devices.findIndex(devices => devices === over?.id);
        console.log(devices);
        const moved = arrayMove(devices, activeIndex, overIndex);
        console.log(moved);
        (await AudioDevicesService.Instance()).setAudioDevices(moved, true);
        setDevices(moved);
    }
  };

  return (
    <div id="systemScene">
      <Card style={{ background: "#ffffff11" }} title="Audio Devices"
        extra={<Button type="primary" icon={<PlusOutlined />} onClick={() => setShowingInputField(true)}>Add</Button>}>
        <DndContext modifiers={[restrictToVerticalAxis]} onDragEnd={onDragEnd}>
          <SortableContext items={devices} strategy={verticalListSortingStrategy}>
            <Table
              dataSource={devices.map(dev => { return { name: dev, key: dev }; })}
              components={{ body: { row: Row } }}
              columns={columns}
              pagination={false} showHeader={false} />
            { showingInputField ? inputField : <></>}
          </SortableContext>
        </DndContext>
      </Card>
    </div>
  )
}
