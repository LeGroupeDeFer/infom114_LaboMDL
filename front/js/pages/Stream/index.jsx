import React, { useState, useEffect } from 'react';
import { Switch, Route, useRouteMatch } from 'react-router-dom';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { Button, ButtonGroup, OverlayTrigger, Tooltip } from 'react-bootstrap';
import clsx from 'clsx';

import { useStream } from 'unanimity/context/streamContext';
import { SearchBar } from 'unanimity/components';
import { kinds, kindOf } from 'unanimity/lib';

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

  const { path } = useRouteMatch();
  const stream = useStream();
  const [state, setState] = useState({
    previewPost: false,
    deletePost: false,
    toast: false,

    onFlag: post => stream.posts.flag(post),
    onHide: post => stream.posts.hide(post),
    onVote: (post, vote) => stream.posts.vote(post, vote),
    onTag: tag => stream.tags.set(tag),
    onSort: order => stream.order.set(order),
    onPreview: v => setState(state => ({ ...state, previewPost: v })),
    onDelete: v => setState(state => ({ ...state, deletePost: v })),
    onToast: v => setState({ ...state, toast: v }),
    onDeleteConfirmation: post => stream.posts.remove(post).then(
      () => setState(state => ({
        ...state,
        deletePost: false,
        toast: false
      }))
    )
  });

  return (
    <>
      <SearchBar>
        <KindSection />
      </SearchBar>

      <Switch>
        <Route exact path={path}>
          <Stream {...state} />
        </Route>
        <Route path={`${path}write`}>
          <Writer {...state} />
        </Route>
        <Route path={`${path}detail/:id`}>
          <Detail {...state} />
        </Route>
      </Switch>

    </>
  );

}


export default StreamContent;
