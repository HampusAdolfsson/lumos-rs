import React, { useState, useEffect } from 'react';
import { ProfileCategory } from './ProfileCategory';
import { Button, Divider, makeStyles, createStyles, Theme, TableContainer, Toolbar, Table, TableBody, Paper, TableRow, Collapse, TableCell, Typography, IconButton } from '@material-ui/core';
import AddIcon from '@material-ui/icons/Add'
import { IProfile, IProfileCategory } from './Profile';
import { ProfilesService } from './ProfilesService';
import { MonitorDialog } from './MonitorDialog';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    divider: {
      marginTop: 10,
      marginBottom: 10,
    },
    profilesScene: {
    },
    button: {
      marginRight: 10,
    },
  }),
);


export function ProfilesScene() {
  const [categories, setCategories] = useState([] as Array<IProfileCategory>);
  const [activeIndex, setActiveIndex] = useState(undefined as (number | undefined));

  useEffect(() => {
    const subscription1 = ProfilesService.Instance.categories.subscribe(cats => setCategories(cats));
    const subscription2 = ProfilesService.Instance.activeProfile.subscribe(index => setActiveIndex(index));
    return () => {
      subscription1.unsubscribe();
      subscription2.unsubscribe();
    };
  });

  const categoryComponents = categories.map((category, i) => {
    return <>
      <ProfileCategory key={category.name} category={category}
        onCategoryChanged={cat => {
          const newCats: IProfileCategory[] = JSON.parse(JSON.stringify(categories));
          newCats[i] = cat;
          ProfilesService.Instance.setProfiles(newCats);
          setCategories(newCats);
        }}
        onCategoryDeleted={() => {
          const newCats: IProfileCategory[] = JSON.parse(JSON.stringify(categories));
          newCats.splice(i, 1);
          ProfilesService.Instance.setProfiles(newCats);
          setCategories(newCats);
        }}
        />
    </>
  });

  const classes = useStyles();
  return (
    <TableContainer component={Paper} className={classes.profilesScene}>
      <Toolbar>
        <Button color="primary" variant="outlined" disableElevation className={classes.button} startIcon={<AddIcon />}
          onClick={() => {
              const newCats = categories.concat([JSON.parse(JSON.stringify(defaultCategory))]);
              ProfilesService.Instance.setProfiles(newCats);
              setCategories(newCats);
            }}>
          Add Category
        </Button>
      </Toolbar>
      <Divider/>
      <Table>
        <TableBody>
          {categoryComponents}
        </TableBody>
      </Table>
    </TableContainer>
  );
}

const defaultCategory: IProfileCategory = {
  name: '',
  profiles: [],
};
