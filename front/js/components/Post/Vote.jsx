import React, { useState } from 'react';

import GoArrowUp from '../../icons/arrow-up.svg';
import GoArrowDown from '../../icons/arrow-down.svg';
import { Button, OverlayTrigger, Tooltip } from 'react-bootstrap';
import clsx from 'clsx';

import { VOTE } from 'unanimity/lib';
import Flexbox from '../Flexbox';
import { May } from '../Auth';

const LockedT = ({ children }) => {
  return (
    <OverlayTrigger
      placement="right"
      overlay={
        <Tooltip>
          Impossible de voter car la publication a été vérouillée par un
          administrateur
        </Tooltip>
      }
    >
      {children}
    </OverlayTrigger>
  );
};

const Hollow = ({ children, setLockedCap }) => {
  setLockedCap(false);
  return <>{children}</>;
};

const Temp = May('post:edit_locked', Hollow, LockedT);

function VoteOverlay({ isLogged, isLocked, children, setLockedCap }) {
  if (isLogged && !isLocked) return <>{children}</>;

  if (isLogged && isLocked)
    return <Temp setLockedCap={setLockedCap}>{children}</Temp>;

  return (
    <OverlayTrigger
      placement="right"
      overlay={<Tooltip>Il faut être authentifié pour pouvoir voter</Tooltip>}
    >
      {children}
    </OverlayTrigger>
  );
}

export function Vote({ isLogged, isLocked, vote, direction, onClick }) {
  const upvote = direction === VOTE.UP;
  const active = vote === direction;
  const cls = clsx('vote p-0', (upvote && 'up') || 'down', active && 'active');
  const Arrow = upvote ? GoArrowUp : GoArrowDown;
  const [lockedCap, setLockedCap] = useState(isLocked);

  return (
    <VoteOverlay
      isLogged={isLogged}
      isLocked={isLocked}
      setLockedCap={setLockedCap}
    >
      <Button
        disabled={!isLogged || lockedCap}
        className={cls}
        onClick={() => onClick(direction, !vote)}
      >
        <Arrow />
      </Button>
    </VoteOverlay>
  );
}

export const UpVote = (props) => <Vote direction={VOTE.UP} {...props} />;

export const DownVote = (props) => <Vote direction={VOTE.DOWN} {...props} />;

export function Score({ score, vote }) {
  const didVote = vote !== VOTE.NONE;
  const cls = clsx('text-center score', didVote && 'active');
  return (
    <div className={cls}>
      <b>{score}</b>
    </div>
  );
}

export function VoteSection({
  isLogged,
  isLocked,
  vote,
  score,
  className,
  onVote,
  ...others
}) {
  const cls = clsx('p-2', 'vote-section', className);
  const localOnVote = (direction, vote) => onVote(vote ? direction : VOTE.NONE);

  return (
    <div className={cls}>
      <Flexbox
        align="center"
        direction="column"
        justify="between"
        className="vote-section-content"
        {...others}
      >
        <UpVote
          isLogged={isLogged}
          vote={vote}
          onClick={localOnVote}
          isLocked={isLocked}
        />
        <Score score={score || 0} vote={vote} />
        <DownVote
          isLogged={isLogged}
          vote={vote}
          onClick={localOnVote}
          isLocked={isLocked}
        />
      </Flexbox>
    </div>
  );
}
