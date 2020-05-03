import React from 'react';
import { GoArrowUp } from 'react-icons/go';
import { GoArrowDown } from 'react-icons/go';
import { Button, OverlayTrigger, Tooltip } from 'react-bootstrap';
import clsx from 'clsx';
import api from '../../lib/api';


export const UpVote = ({ is_logged, voted, set_vote, score, set_score, post_id }) => {
  let notLoggedMsg = 'Il faut être authentifié pour pouvoir voter';

  function upVote(e, cancel) {
    e.stopPropagation();

    const vote = () => {
      let vote = 1;
      if (cancel) {
        vote = 0;
      }
      api.vote(post_id, vote).then(() => {
        if (cancel) {
          set_score(score - 1);
          set_vote('no');
        } else {
          // Case : We directly go from down to up
          if (voted == 'down') {
            set_score(score + 2);
          } else {
            set_score(score + 1);
          }
          set_vote('up');
        }
      }).catch((error) => {
        console.log(error);
      });
    }
    vote();
  }

  return (
    <>
      {is_logged ? (
        <Button
          variant="light"
          className={`up-vote-btn ${clsx(voted === 'up' && 'up-voted')}`}
          onClick={(e) => upVote(e, voted === 'up')}
        >
          <GoArrowUp size="1.5em" />
        </Button>
      ) : (
        <OverlayTrigger
          placement="right"
          overlay={<Tooltip> {notLoggedMsg} </Tooltip>}
        >
            <span className="d-inline-block">
              <Button variant="light" className={'up-vote-btn'} disabled>
                <GoArrowUp size="1.5em" />
              </Button>
            </span>
        </OverlayTrigger>
      )}
    </>
  );
};

UpVote.defaultProps = {
  is_logged: false,
  click_handle: null,
  voted: 'no',
};

export const DownVote = ({ is_logged, voted, set_vote, score, set_score, post_id }) => {
  let notLoggedMsg = 'Il faut être authentifié pour pouvoir voter';

  function downVote(e, cancel) {
    e.stopPropagation();

    const vote = () => {
      let vote = -1;
      if (cancel) {
        vote = 0;
      }
      api.vote(post_id, vote).then(() => {
        if (cancel) {
          set_score(score + 1);
          set_vote('no');
        } else {
          // Case : We directly go from up to down
          if (voted == 'up') {
            set_score(score - 2);
          } else {
            set_score(score - 1);
          }
          set_vote('down');
        }
      }).catch((error) => {
        console.log(error);
      });
    }

    vote();
  }

  return (
    <>
      {is_logged ? (
        <Button
          variant="light"
          className={`down-vote-btn ${clsx(voted === 'down' && 'down-voted')}`}
          onClick={(e) => downVote(e, voted === 'down')}
        >
          <GoArrowDown size="1.5em" />
        </Button>
      ) : (
        <OverlayTrigger
          placement="right"
          overlay={<Tooltip> {notLoggedMsg} </Tooltip>}
        >
            <span className="d-inline-block">
              <Button variant="light" className={'down-vote-btn'} disabled>
                <GoArrowDown size="1.5em" />
              </Button>
            </span>
        </OverlayTrigger>
      )}
    </>
  );
};

DownVote.defaultProps = {
  is_logged: false,
  click_handle: null,
  voted: 'no',
};
