import React, { useState } from 'react';
import update from 'immutability-helper';
import Slider from '@material-ui/core/Slider';
import { useMutation } from '@apollo/react-hooks';
import { gql } from "apollo-boost";
import Title from './Title';

const SET_LIGHT = gql`
  mutation SetLight($lightId: LightId!, $r: Float!, $g: Float!, $b: Float!) {
    manualMode(settings: {id: $lightId, r: $r, g: $g, b: $b})
  }
`;

export default function ManualLightSetter(props) {
  const [setLight, { data }] = useMutation(SET_LIGHT);
  const [state, setState] = useState({lightId: props.lightId, "r": 0, g: 0, b: 0});

  const handleRedChange = (event, newValue) => {
    setState(update(state, {r: {$set: newValue}}));
  };

  const handleGreenChange = (event, newValue) => {
    setState(update(state, {g: {$set: newValue}}));
  };

  const handleBlueChange = (event, newValue) => {
    setState(update(state, {b: {$set: newValue}}));
  };

  const handleSlideFinished = (event, newValue) => {
    setLight({variables: state});
  };

  return (
    <React.Fragment>
      <Title>{state.lightId}</Title>
        <Slider min={0} max={1} step={0.01} value={state.r} onChange={handleRedChange} onChangeCommitted={handleSlideFinished}/>
        <Slider min={0} max={1} step={0.01} value={state.g} onChange={handleGreenChange} onChangeCommitted={handleSlideFinished}/>
        <Slider min={0} max={1} step={0.01} value={state.b} onChange={handleBlueChange} onChangeCommitted={handleSlideFinished}/>
    </React.Fragment>
  )
}
