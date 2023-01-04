import { useState } from "react";
import { DeleteFilled, SettingFilled } from "@ant-design/icons";
import { Button, Space } from "antd";
import { ProfileSettings } from "./ProfileSettings";
import { IProfile } from "./Profile";

interface Props {
  profile: IProfile;
  onProfileChanged: (profile: IProfile) => void;
  onProfileDeleted: () => void;
}

export default function ProfileEntryActions(props: Props) {
  const [dialogOpen, setDialogOpen] = useState(false);

  return <>
    <Space>
      <Button type="default" onClick={() => { setDialogOpen(true); }}
        icon={<SettingFilled />} />
      <Button danger
        icon={<DeleteFilled />} onClick={props.onProfileDeleted} />
    </Space>
    <ProfileSettings open={dialogOpen} onClosed={() => setDialogOpen(false)}
        onProfileChanged={(profile) => { setDialogOpen(false); props.onProfileChanged(profile); }}
        profile={props.profile}/>
  </>;
}
