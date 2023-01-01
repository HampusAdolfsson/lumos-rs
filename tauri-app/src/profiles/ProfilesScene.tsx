import { useState, useEffect } from 'react';
import { ProfileCategory } from './ProfileCategory';
import { Button, Divider, TableContainer, Toolbar, Table, TableBody, Paper } from '@mui/material';
import AddIcon from '@mui/icons-material/Add'
import { IProfileCategory } from './Profile';
import { ProfilesService } from './ProfilesService';

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

  const categoryComponents = categories.map((category, i) => {
    return <>
      <ProfileCategory key={category.name} category={category} activeProfiles={activeProfiles}
        onCategoryChanged={async cat => {
          const newCats: IProfileCategory[] = JSON.parse(JSON.stringify(categories));
          newCats[i] = cat;
          (await ProfilesService.Instance()).setProfiles(newCats);
          setCategories(newCats);
        }}
        onCategoryDeleted={async() => {
          const newCats: IProfileCategory[] = JSON.parse(JSON.stringify(categories));
          newCats.splice(i, 1);
          (await ProfilesService.Instance()).setProfiles(newCats);
          setCategories(newCats);
        }}
        />
    </>
  });

  return (
    <TableContainer component={Paper}>
      <Table>
        <TableBody>
          {categoryComponents}
        </TableBody>
      </Table>
      <Divider/>
      <Toolbar>
        <Button color="primary" variant="outlined" disableElevation sx={{ marginRight: 10 }} startIcon={<AddIcon />}
        onClick={async() => {
        const newCats = categories.concat([JSON.parse(JSON.stringify(defaultCategory))]);
        (await ProfilesService.Instance()).setProfiles(newCats);
        setCategories(newCats);
        }}>
        Add Category
        </Button>
      </Toolbar>
    </TableContainer>
  );
}

const defaultCategory: IProfileCategory = {
  name: '',
  profiles: [],
  priority: 0,
  enabled: true,
};
