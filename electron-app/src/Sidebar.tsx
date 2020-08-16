import React from 'react';
import { Typography } from '@material-ui/core';

interface State {
}

interface Props {
  scenes: string[];
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
        <Typography variant="h1" color={index === this.props.selectedScene ? "primary" : "textPrimary"}
          onClick={handler} key={scene}>
          {scene}
        </Typography>
      );
    });
    return (
      <div className="sidebar">
        {scenes}
      </div>
    );
  }
}