import { useState } from 'react';
import { IAreaSpecification, IProfile, IProfileCategory, MonitorDistance } from './Profile';
import { Button, Table, TableColumnsType } from 'antd';
import ProfileEntryActions from './ProfileEntryActions';
import { ProfilesService } from './ProfilesService';
import { PlusOutlined } from '@ant-design/icons';

interface Props {
  category: IProfileCategory;
  onCategoryChanged: (category: IProfileCategory) => void;
}

export function ProfileCategory(props: Props) {
  const [dialogOpen, setDialogOpen] = useState(false);

  const columns: TableColumnsType<IProfile> = [
    {
      key: "name",
      render: (_, profile) => {
        return <>{profile.regex || "New Profile"}</>
      },
    },
    {
      key: "size",
      render: (_, profile) => <>{profile.areas.map(areaSize).join(", ")}</>
    },
    {
      key: "actions",
      align: "right",
      render: (_, profile, i) => <ProfileEntryActions profile={profile} onProfileDeleted={() => {
        const newCategory = JSON.parse(JSON.stringify(props.category));
        newCategory.profiles.splice(i, 1);
        props.onCategoryChanged(newCategory);
      }} onProfileChanged={profile => {
        const newCategory = JSON.parse(JSON.stringify(props.category));
        newCategory.profiles[i] = profile;
        props.onCategoryChanged(newCategory);
      }} />
    }
  ];

  return (
    <>
      <Table dataSource={props.category.profiles} columns={columns} showHeader={false} pagination={false} />
      <Button type="primary" icon={<PlusOutlined/>} style={{ marginTop: 10 }}
        onClick={async() => {
          const newCategory = JSON.parse(JSON.stringify(props.category));
          newCategory.profiles.push(JSON.parse(JSON.stringify((await ProfilesService.Instance()).createProfile())));
          props.onCategoryChanged(newCategory);
        }}>
        Add Profile
      </Button>
    </>
  );
}

function areaSize(area: IAreaSpecification) {
  const distToStr = (dist: MonitorDistance) => {
    if ("px" in dist) {
      return dist.px.toString();
    } else {
      return dist.percentage + "%";
    }
  }

  return distToStr(area.width) + " x " + distToStr(area.height);
}