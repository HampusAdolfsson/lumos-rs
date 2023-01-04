import React, { useState } from 'react';
import { IProfile } from './Profile';
import { AreaSpecificationsParser, composeAreaSpecifications } from './parsing/AreaSpecificationParser';
import { tokenize } from './parsing/Lexer';
import { StringReader } from './parsing/StringReader';
import { Alert, Input, InputNumber, Modal, Space } from 'antd';

export interface Props {
  profile: IProfile;
  open: boolean;
  onProfileChanged: (profile: IProfile) => void;
  onClosed: () => void;
}

export function ProfileSettings(props: Props) {
  const [profile, setProfile] = useState(props.profile);
  const [areaSpecification, setAreaSpecification] = useState(composeAreaSpecifications(props.profile.areas));
  const [dirty, setDirty] = useState(false);
  const [errorMsg, setErrorMsg] = useState("");


  const handleInput = (event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>) => {
    const target = event.target;
    const name = target.name;
    let newProfile = JSON.parse(JSON.stringify(profile));
    newProfile[name] = target.value;
    setProfile(newProfile);
    setDirty(true);
  }

  const [priority, setPriority] = useState(props.profile.priority?.toString() ?? "");

  return (
    <>
      <Modal open={props.open} onCancel={props.onClosed} onOk={() => {
        try {
          const areas = new AreaSpecificationsParser().parse(tokenize(new StringReader(areaSpecification)));
          profile.areas = areas;
          profile.priority = priority === "" || priority === undefined ? undefined : Number(priority);
          setDirty(false);
          props.onProfileChanged(profile);
          setErrorMsg("");
        } catch(e) {
          setErrorMsg(e as string);
        }
      }} okText="Save" title="Profile Settings">
        <Space direction="vertical" style={{ width: "100%" }}>
        <Input placeholder="Window Title Regex" value={profile.regex} name="regex" onChange={handleInput} style={{ width: "100%" }}/>
          <Input.TextArea autoSize placeholder="Definition(s)" value={areaSpecification} onChange={ev => {
            setAreaSpecification(ev.target.value);
            setDirty(true);
          }} style={{ width: "100%" }} />
          { errorMsg ? <Alert message={errorMsg} type="error" showIcon/> : <></> }
          <InputNumber placeholder="Priority" value={priority} onChange={value => {setPriority(value ?? ""); setDirty(true);}}
            style={{ width: "100%" }}/>
        </Space>
      </Modal>
    </>
  );
}