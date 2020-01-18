import React from 'react';
import ListItem from '@material-ui/core/ListItem';
import ListItemIcon from '@material-ui/core/ListItemIcon';
import ListItemText from '@material-ui/core/ListItemText';
import ListSubheader from '@material-ui/core/ListSubheader';
import BrushIcon from '@material-ui/icons/Brush';
import PowerSettingsNewIcon from '@material-ui/icons/PowerSettingsNew';
import FavoriteIcon from '@material-ui/icons/Favorite';

export const mainListItems = (
  <div>
    <ListSubheader inset>Modes</ListSubheader>
    <ListItem button>
      <ListItemIcon>
        <PowerSettingsNewIcon />
      </ListItemIcon>
      <ListItemText primary="Lights Off" />
    </ListItem>
    <ListItem button>
      <ListItemIcon>
        <BrushIcon />
      </ListItemIcon>
      <ListItemText primary="Manual Settings" />
    </ListItem>
    <ListItem button>
      <ListItemIcon>
        <FavoriteIcon />
      </ListItemIcon>
      <ListItemText primary="PinkPulse" />
    </ListItem>
  </div>
);
