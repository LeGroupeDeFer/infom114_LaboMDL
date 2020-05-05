import React, {useState, useMemo, useEffect} from 'react';
import { Switch, Route, useRouteMatch } from 'react-router-dom';

import Stream from './Stream';
import Writer from './Writer';
import Detail from './Detail';
import { SearchBar } from 'unanimity/components';
import { usePositiveEffect, useRequest } from 'unanimity/hooks';
import {head, api, trace, equal} from 'unanimity/lib';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import {
  faBalanceScale,
  faGlobeEurope,
  faInfo,
  faLightbulb,
} from '@fortawesome/free-solid-svg-icons';
import { Button, ButtonGroup, OverlayTrigger, Tooltip } from 'react-bootstrap';
import clsx from 'clsx';

const SORT = Object.freeze({
  RANK: Object.freeze({ DESC: 'high_rank', ASC: 'low_rank' }),
  SCORE: Object.freeze({ DESC: 'low', ASC: 'high' }),
  VOTES: Object.freeze({ DESC: 'top', ASC: 'low' }),
  AGE: Object.freeze({ DESC: 'new', ASC: 'old' }),
});

const KIND = Object.freeze({
  ALL: { label: 'Actualité', key: 'all', icon: faGlobeEurope },
  INFO: { label: 'Infos', key: 'info', icon: faInfo },
  IDEA: { label: 'Idées', key: 'idea', icon: faLightbulb },
  POLL: { label: 'Sondages', key: 'poll', icon: faBalanceScale },
});

const kinds = Object.values(KIND);
const kindOf = (key) =>
  head(
    Object.keys(KIND)
      .map((k) => KIND[k])
      .filter((k) => k.key === key)
  );

// FilterBar :: Object => Component
function KindSection({ kind, onChange }) {
  return (
    <ButtonGroup className="kind-section d-flex justify-content-between">
      {kinds.map(({ key, icon, label }) => (
        <OverlayTrigger
          key={key}
          placement="bottom"
          overlay={<Tooltip>{label}</Tooltip>}
        >
          <Button
            key={key}
            className={clsx('kind-choice', kind.key === key && 'active')}
            onClick={() => onChange(kindOf(key))}
          >
            <Icon icon={icon} />
          </Button>
        </OverlayTrigger>
      ))}
    </ButtonGroup>
  );
}

function InnerStreamContent({ tags, selectedTags, selectedKind }) {
  console.log('RENDER');

  const { path } = useRouteMatch();
  const [sort, setSort] = useState(SORT.RANK.DESC);
  const [writtenPost, setWrittenPost] = useState(null);

  /* Fetch posts */
  const [promise, setPromise] = useState(null);
  const [posts, setPosts] = useState([]);
  const [error, setError] = useState(null);

  const queryTags = selectedTags.length ? { tags: selectedTags } : {};
  const queryKind = selectedKind !== KIND.ALL ? { kind: selectedKind.key } : {};
  const query = { sort, ...queryTags, ...queryKind };
  const [localQuery, setLocalQuery] = useState(query);

  if (!equal(query, localQuery))
    setLocalQuery(query);

  useEffect(() => setPromise(api.posts.where(query)), [localQuery]);

  usePositiveEffect(() => {
    let isSubscribed = true;
    promise
      .then(data => isSubscribed ? setPosts(data) : undefined)
      .catch(error => isSubscribed ? setError(error) : undefined)
      .finally(() => setPromise(null));
    return () => isSubscribed = false;
  }, [promise]);
  /* End fetch posts */

  return (
    <Switch>
      <Route exact path={path}>
        <Stream posts={posts} onSort={setSort} kind={selectedKind} />
      </Route>
      <Route path={`${path}write`}>
        <Writer onWrite={setWrittenPost} />
      </Route>
      <Route path={`${path}detail/:id`}>
        <Detail post={writtenPost} />
      </Route>
    </Switch>
  );
}

// StreamContent :: None => Component
let j = 0;
function StreamContent() {
  /* Fetch tags */
  const [promise, setPromise] = useState(null);
  const [tags, setTags] = useState([]);
  const [error, setError] = useState(null);

  useEffect(() => setPromise(api.tags()), []);

  usePositiveEffect(() => {
    let isSubscribed = true;
    promise
      .then(({ tags }) => isSubscribed ? setTags(tags) : undefined)
      .catch(error => isSubscribed ? setError(error) : undefined)
      .finally(() => setPromise(null));
    return () => isSubscribed = false;
  }, [promise]);

  /* End fetch tags */

  const [selectedTags, setSelectedTags] = useState([]);
  const [selectedKind, setSelectedKind] = useState(KIND.ALL);

  return (
    <>
      <SearchBar
        onChange={setSelectedTags}
        tags={tags}
        selectedTags={selectedTags}
      >
        <KindSection onChange={setSelectedKind} kind={selectedKind} />
      </SearchBar>
      <InnerStreamContent
        tags={tags}
        selectedTags={selectedTags}
        selectedKind={selectedKind}
      />
    </>
  );
}

export default StreamContent;
