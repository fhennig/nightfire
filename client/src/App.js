import React, { useState } from 'react';
import update from 'immutability-helper';
import Slider from 'react-rangeslider';
import 'react-rangeslider/lib/index.css';
import './App.css';
import ApolloClient from 'apollo-boost';
import { ApolloProvider, useQuery, useMutation } from '@apollo/react-hooks';
import { gql } from "apollo-boost";

const client = new ApolloClient({
  uri: '/graphql',
});

const SET_LIGHT = gql`
  mutation SetLight($lightId: String!, $r: Float!, $g: Float!, $b: Float!) {
    setLight(id: $lightId, color: {r: $r, g: $g, b: $b}) {
      id
    }
  }
`;

const ColorControl = (props) => {
  const [setLight, { data }] = useMutation(SET_LIGHT);
  const [state, setState] = useState({lightId: props.lightId, "r": 0, g: 0, b: 0});

  const handleRedChange = (value) => {
    setState(update(state, {r: {$set: value}}));
  };

  const handleGreenChange = (value) => {
    setState(update(state, {g: {$set: value}}));
  };

  const handleBlueChange = (value) => {
    setState(update(state, {b: {$set: value}}));
  };

  const handleSlideFinished = (value) => {
    setLight({variables: state});
  };

  return (
    <div className="light-control">
      {state.lightId}
      <ul>
        <li><Slider min={0} max={1} step={0.01} value={state.r} onChange={handleRedChange} onChangeComplete={handleSlideFinished}/></li>
        <li><Slider min={0} max={1} step={0.01} value={state.g} onChange={handleGreenChange} onChangeComplete={handleSlideFinished}/></li>
        <li><Slider min={0} max={1} step={0.01} value={state.b} onChange={handleBlueChange} onChangeComplete={handleSlideFinished}/></li>
      </ul>
    </div>
  )
};

function App() {
  return (
    <div className="App">
      <ApolloProvider client={client}>
        <header className="App-header">
          <ColorControl lightId="light1" />
          <ColorControl lightId="light2" />
          <ColorControl lightId="light3" />
          <ColorControl lightId="light4" />
        </header>
      </ApolloProvider>
    </div>
  );
}

export default App;
