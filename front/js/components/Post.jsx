import React, { useState, useEffect } from 'react';
import Card from 'react-bootstrap/Card';
import { dev, preview } from '../lib';
import Badge from 'react-bootstrap/Badge';
import Button from 'react-bootstrap/Button';
import Moment from 'react-moment';
import { GoArrowDown, GoArrowUp } from 'react-icons/go';
import { MdModeComment, MdReport } from 'react-icons/md';
import {
  FaTag,
  FaFacebookSquare,
  FaEllipsisH,
  FaEyeSlash,
  FaFlag
} from 'react-icons/fa';
import clsx from 'clsx';
import Tooltip from 'react-bootstrap/Tooltip';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import DropdownButton from 'react-bootstrap/DropdownButton';
import Dropdown from 'react-bootstrap/Dropdown';

const Post = ({
  title,
  text,
  username,
  voteCount,
  type,
  previewLength,
  createdOn,
  currentFilter,
  ...otherProps
}) => {
  const [voted, setVoted] = useState('no');
  const [voteCountState, setvoteCountState] = useState(voteCount);

  useEffect(() => {
    setvoteCountState(voteCount);
  }, [voteCount]);

  function upVote(cancel) {
    if (cancel) {
      setvoteCountState(voteCountState - 1);
      setVoted('no');
    } else {
      // Case : We directly go from down to up
      if (voted == 'down') {
        setvoteCountState(voteCountState + 2);
      } else {
        setvoteCountState(voteCountState + 1);
      }
      setVoted('up');
    }
  }

  function downVote(cancel) {
    if (cancel) {
      setvoteCountState(voteCountState + 1);
      setVoted('no');
    } else {
      // Case : We directly go from down to up
      if (voted == 'up') {
        setvoteCountState(voteCountState - 2);
      } else {
        setvoteCountState(voteCountState - 1);
      }
      setVoted('down');
    }
  }

  function getDisplayedType(type) {
    switch (type) {
      case 'info':
        return 'Information';
      case 'poll':
        return 'Vote';
      case 'idea':
        return 'Idée';
    }
  }

  if (!['all', type].includes(currentFilter)) return <></>;
  let upVoteBtn;
  let downVoteBtn;

  if (otherProps.is_logged) {
    upVoteBtn = (
      <Button
        variant="light"
        className={`up-vote-btn ${clsx(voted === 'up' && 'up-voted')}   `}
        onClick={() => upVote(voted === 'up')}
      >
        <GoArrowUp size="1.5em" />
      </Button>
    );

    downVoteBtn = (
      <Button
        variant="light"
        className={`down-vote-btn ${clsx(voted === 'down' && 'down-voted')}`}
        onClick={() => downVote(voted === 'down')}
      >
        <GoArrowDown size="1.5em" />
      </Button>
    );
  } else {
    let notLoggedMsg = 'Il faut être authentifié pour pouvoir voter';
    upVoteBtn = (
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
    );

    downVoteBtn = (
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
    );
  }

  function handleHoverOff() {
    console.log('Hover off post');
  }

  return (
    <div className="d-flex">
      <Card {...otherProps} className="post" onMouseLeave={handleHoverOff}>
        <Card.Header>
          <h5>
            <Badge className={`post-${type} mr-2`}>
              {getDisplayedType(type)}
            </Badge>
            <a href="#" className="text-dark">
              {username}
            </a>
            <span className="text-muted">
              {' '}
              -{' '}
              <Moment locale="fr" fromNow>
                {createdOn}
              </Moment>
            </span>

            <DropdownButton
              title={
                <span>
                  <FaEllipsisH />
                </span>
              }
              variant="link"
              className="float-right more btn-link"
            >
              <Dropdown.Item as="button">
                <FaEyeSlash className="mr-2" />
                Masquer
              </Dropdown.Item>
              <Dropdown.Item as="button">
                <FaFlag className="mr-2" />
                Signaler
              </Dropdown.Item>
            </DropdownButton>
          </h5>
        </Card.Header>

        <Card.Body className="p-0">
          <div className="d-flex">
            <div className="vote-section px-3">
              {upVoteBtn}

              <div
                className={`text-center ${clsx(
                  voted !== 'no' && voted + '-voted'
                )}`}
              >
                <b>{voteCountState}</b>
              </div>

              {downVoteBtn}
            </div>

            <div className="p-3">
              <Card.Title className="mb-1">{title}</Card.Title>
              <div className="mb-1">
                <a
                  href="#"
                  className="mr-2 tag"
                  onClick={e => otherProps.tag_click(e)}
                  value="Arsenal"
                >
                  <FaTag className="mr-1" />
                  Arsenal
                </a>
                <a
                  href="#"
                  className="mr-2 tag"
                  onClick={e => otherProps.tag_click(e)}
                  value="FacInfo"
                >
                  <FaTag className="mr-1" />
                  FacInfo
                </a>
                <a
                  href="#"
                  className="mr-2 tag"
                  onClick={e => otherProps.tag_click(e)}
                  value="FacEco"
                >
                  <FaTag className="mr-1" />
                  FacEco
                </a>
              </div>

              <Card.Text>
                {preview(text, previewLength)} <a href="#">Read more</a>
              </Card.Text>
              <a className="post-footer-btn mr-2" href="#">
                <MdModeComment size="1.25em" className="mr-1" />
                <span className="text-muted">12 commentaires</span>
              </a>
              <a className="post-footer-btn mr-2" href="#">
                <FaFacebookSquare size="1.25em" className="mr-1" />
                <span className="text-muted">Partager</span>
              </a>
            </div>
          </div>
        </Card.Body>
      </Card>
    </div>
  );
};

Post.defaultProps = {
  title: 'A post',
  text: dev.loremIpsum,
  username: 'John Coffey',
  previewLength: 200,
  voteCount: 25,
  type: 'info',
  createdOn: '2020-02-29T12:59-0500'
};

export default Post;
