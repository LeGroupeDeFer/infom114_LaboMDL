import React, { useState, useEffect } from 'react';
import { Switch, Route, useRouteMatch } from 'react-router-dom';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { Button, ButtonGroup, OverlayTrigger, Tooltip } from 'react-bootstrap';
import clsx from 'clsx';

import { usePositiveEffect } from 'unanimity/hooks';
import { StreamProvider, useStream } from 'unanimity/context/streamContext';
import { SearchBar } from 'unanimity/components';
import { api, kinds, kindOf } from 'unanimity/lib';

import Stream from './Stream';
import Writer from './Writer';
import Detail from './Detail';


// FilterBar :: Object => Component
function KindSection() {
  const stream = useStream();

  return (
    <ButtonGroup className="kind-section d-flex justify-content-between">
      {kinds.map(({ key, icon, label }) => (
        <OverlayTrigger
          key={key}
          placement="bottom"
          overlay={<Tooltip id={key}>{label}</Tooltip>}
        >
          <Button
            key={key}
            className={clsx('kind-choice', stream.kind.value.key === key && 'active')}
            onClick={() => stream.kind.set(kindOf(key))}
          >
            <Icon icon={icon} />
          </Button>
        </OverlayTrigger>
      ))}
    </ButtonGroup>
  );

}

// StreamContent :: None => Component
function StreamContent() {

  /* Fetch tags TODO MOVE IN STREAM CONTEXT */
  const { path } = useRouteMatch();

  return (
    <StreamProvider>

      <SearchBar>
        <KindSection />
      </SearchBar>

      <Switch>
        <Route exact path={path}>
          <Stream />
        </Route>
        <Route path={`${path}write`}>
          <Writer />
        </Route>
        <Route path={`${path}detail/:id`}>
          <Detail />
        </Route>
      </Switch>

    </StreamProvider>
  );

}


export default StreamContent;
