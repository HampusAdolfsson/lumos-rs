import React from 'react';
import { List, ListItem, ListItemIcon, ListItemText, Typography } from '@material-ui/core';
import { DevicesOther } from '@material-ui/icons';

interface State {
}

interface Props {
  scenes: { name: string, icon: JSX.Element }[];
  selectedScene: number;
  onSceneChanged: (i: number) => void;
}

export class Sidebar extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = {
      selectedScene: 0,
    }
  }

  setSelected(i: number) {
    this.setState({
      selectedScene: i,
    });
  }

  render() {
    const scenes = this.props.scenes.map((scene, index) => {
      const handler = () => this.props.onSceneChanged(index);
      return (
        <ListItem button>
          <ListItemIcon>
            {scene.icon}
          </ListItemIcon>
          <ListItemText primary={scene.name}/>
        </ListItem>
        // <Typography variant="h1" color={index === this.props.selectedScene ? "primary" : "textPrimary"}
        //   onClick={handler} key={scene}>
        //   {scene}
        // </Typography>
      );
    });
    return (
      <div className="sidebar">
        <List component="nav">
          {scenes}
        </List>
      </div>
    );
  }
}