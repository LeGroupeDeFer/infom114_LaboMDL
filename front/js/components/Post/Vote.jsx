import React from 'react';

import GoArrowUp from '../../icons/arrow-up.svg';
import GoArrowDown from '../../icons/arrow-down.svg';
import { Button, OverlayTrigger, Tooltip } from 'react-bootstrap';
import clsx from 'clsx';

import { VOTE } from 'unanimity/lib';
import Flexbox from '../Flexbox';

function VoteOverlay({ isLogged, isLocked, children }) {
  if (isLogged && !isLocked) return <>{children}</>;

  if (!isLogged)
    return (
      <OverlayTrigger
        placement="right"
        overlay={<Tooltip>Il faut être authentifié pour pouvoir voter</Tooltip>}
      >
        {children}
      </OverlayTrigger>
    );

  if (isLocked)
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
}

export function Vote({ isLogged, isLocked, vote, direction, onClick }) {
  const upvote = direction === VOTE.UP;
  const active = vote === direction;
  const cls = clsx('vote p-0', (upvote && 'up') || 'down', active && 'active');
  const Arrow = upvote ? GoArrowUp : GoArrowDown;

  return (
    <VoteOverlay isLogged={isLogged} isLocked>
      <Button
        disabled={!isLogged || isLocked}
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
  const cls = clsx('text-center', didVote && 'active');
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
          isLocked
        />
        <Score score={score || 0} vote={vote} />
        <DownVote
          isLogged={isLogged}
          vote={vote}
          onClick={localOnVote}
          isLocked
        />
      </Flexbox>
    </div>
  );
}
