import { useState, useEffect } from 'react';
import { ProfileCategory } from './ProfileCategory';
import { IProfileCategory } from './Profile';
import { ProfilesService } from './ProfilesService';
import { Button, Card, Space, Switch, Table, TableColumnsType } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import ProfileCategoryEntryActions from './ProfileCategoryEntryActions';

export function ProfilesScene() {
  const [categories, setCategories] = useState([] as Array<IProfileCategory>);
  const [activeProfiles, setActiveProfiles] = useState(new Map<number, number>());

  useEffect(() => {
    const subscription1 = ProfilesService.Instance().then(service => service.categories.subscribe(cats => setCategories(cats)));
    const subscription2 = ProfilesService.Instance().then(service => service.activeProfiles.subscribe(map => setActiveProfiles(map)));
    return () => {
      subscription1.then(sub => sub.unsubscribe());
      subscription2.then(sub => sub.unsubscribe());
    };
  }, []);

  const columns: TableColumnsType<IProfileCategory> = [
    {
      key: "name",
      render: (_, category, i) => (<Space>
        <span>{category.name}</span>
        <Switch size="small" checked={category.enabled} onChange={async checked => {
          const newCategory = JSON.parse(JSON.stringify(category));
          newCategory.enabled = checked;
          const newCats: IProfileCategory[] = JSON.parse(JSON.stringify(categories));
          newCats[i] = newCategory;
          (await ProfilesService.Instance()).setProfiles(newCats);
          setCategories(newCats);
        }}/>
      </Space>),
    },
    {
      key: "size",
      render: (_, category) => (<>{category.profiles.length} profile(s)</>)
    },
    {
      key: "actions",
      align: "right",
      render: (_, category, i) => (<ProfileCategoryEntryActions category={category} onCategoryDeleted={async() => {
          const newCats: IProfileCategory[] = JSON.parse(JSON.stringify(categories));
          newCats.splice(i, 1);
          (await ProfilesService.Instance()).setProfiles(newCats);
          setCategories(newCats);
      }} onCategoryChanged={async(cat) => {
        const newCats: IProfileCategory[] = JSON.parse(JSON.stringify(categories));
        newCats[i] = cat;
        (await ProfilesService.Instance()).setProfiles(newCats);
        setCategories(newCats);
      }}/>),
    }
  ];

  const expandedRowRender = (category: IProfileCategory, i: number) => (
        <ProfileCategory category={category} onCategoryChanged={async category => {
          const newCats: IProfileCategory[] = JSON.parse(JSON.stringify(categories));
          newCats[i] = category;
          (await ProfilesService.Instance()).setProfiles(newCats);
          setCategories(newCats);
        }}/>
      );

  return (
    <Card style={{ background: "#ffffff11" }} title="Profile Categories" extra={<Button type="primary" icon={<PlusOutlined/>} onClick={async() => {
      const newCats = categories.concat([JSON.parse(JSON.stringify(defaultCategory))]);
      (await ProfilesService.Instance()).setProfiles(newCats);
      setCategories(newCats);
    }}>Add</Button>}>
      <Table dataSource={categories.map((cat, i) => { return {...cat, key: i}; })} columns={columns}
        pagination={false} showHeader={false} expandable={{ expandedRowRender }} />
    </Card>
  );
}

const defaultCategory: IProfileCategory = {
  name: '',
  profiles: [],
  priority: 0,
  enabled: true,
};
