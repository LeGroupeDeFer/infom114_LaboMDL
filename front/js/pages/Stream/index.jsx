import React, { useState, useMemo } from 'react';
import { Switch, Route, useRouteMatch } from 'react-router-dom';

import Stream from './Stream';
import Writer from './Writer';
import Detail from './Detail';
import { SearchBar } from 'unanimity/components';
import { useRequest } from 'unanimity/hooks';
import { head, api, trace } from 'unanimity/lib';
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
  const { path } = useRouteMatch();
  const [sort, setSort] = useState(SORT.RANK.DESC);
  const [writtenPost, setWrittenPost] = useState(null);

  const queryTags = selectedTags.length ? { tags: selectedTags } : {};
  const queryKind = selectedKind !== KIND.ALL ? { kind: selectedKind.key } : {};
  const currentQuery = { sort, ...queryTags, ...queryKind };
  const [error, posts] = useRequest(api.posts.where, [currentQuery], []);

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
function StreamContent() {
  const [error, { tags }] = useRequest(api.tags, [], { tags: [] });

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
