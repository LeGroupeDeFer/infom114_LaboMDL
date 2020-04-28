import React, { useState, useEffect } from 'react';
import Card from 'react-bootstrap/Card';
import { dev, preview } from '../lib';
import Badge from 'react-bootstrap/Badge';
import GoArrowDown from 'react-icons/go';
import Moment from 'react-moment';
import DownVote from './DownVote';
import UpVote from './UpVote';
import { MdModeComment, MdReport } from 'react-icons/md';
import {
  FaTag,
  FaFacebookSquare,
  FaEllipsisH,
  FaEyeSlash,
  FaFlag,
} from 'react-icons/fa';
import clsx from 'clsx';
import DropdownButton from 'react-bootstrap/DropdownButton';
import Dropdown from 'react-bootstrap/Dropdown';
import { FacebookShareButton } from 'react-share';

const PostPreview = ({
  title,
  text,
  username,
  points,
  type,
  previewLength,
  createdOn,
  currentFilter,
  commentNb,
  ...otherProps
}) => {
  const [voted, setVoted] = useState('no');
  const [pointsState, setPointsState] = useState(points);

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

  return (
    <div className="d-flex">
      <Card
        {...otherProps}
        className="post"
        onClick={(e) => otherProps.show_modal(e)}
      >
        <Card.Header>
          <h5>
            <Badge className={`post-${type} mr-2`}>
              {getDisplayedType(type)}
            </Badge>
            <span className="mr-2">{title}</span>

            <span className="text-muted">
              {' '}
              <a href="#" className="text-dark">
                {username}
              </a>{' '}
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
              onClick={(e) => e.stopPropagation()}
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
            <div className="vote-section">
              <UpVote
                is_logged={otherProps.is_logged}
                voted={voted}
                set_vote={setVoted}
                points={pointsState}
                set_points={setPointsState}
              />
              <div
                className={`text-center ${clsx(
                  voted !== 'no' && voted + '-voted'
                )}`}
              >
                <b>{pointsState}</b>
              </div>

              <DownVote
                is_logged={otherProps.is_logged}
                voted={voted}
                set_vote={setVoted}
                points={pointsState}
                set_points={setPointsState}
              />
            </div>

            <div className="px-3 pb-3 pt-2">
              <div className="mb-1">
                <a
                  href="#"
                  className="mr-2 tag"
                  onClick={(e) => otherProps.tag_click(e)}
                  value="Arsenal"
                >
                  <FaTag className="mr-1" />
                  Arsenal
                </a>
                <a
                  href="#"
                  className="mr-2 tag"
                  onClick={(e) => otherProps.tag_click(e)}
                  value="FacInfo"
                >
                  <FaTag className="mr-1" />
                  FacInfo
                </a>
                <a
                  href="#"
                  className="mr-2 tag"
                  onClick={(e) => otherProps.tag_click(e)}
                  value="FacEco"
                >
                  <FaTag className="mr-1" />
                  FacEco
                </a>
              </div>

              <Card.Text>
                {preview(text, previewLength)} <a href="#">Lire la suite</a>
              </Card.Text>

              <a className="post-footer-btn mr-2" href="#">
                <MdModeComment size="1.25em" className="mr-1" />
                <span className="text-muted">
                  {commentNb} {commentNb <= 1 ? 'commentaire' : 'commentaires'}
                </span>
              </a>

              <FacebookShareButton
                url="unanimty.be"
                quote="Vive le covid-19"
                onClick={(e) => e.stopPropagation()}
              >
                <a className="post-footer-btn mr-2" href="#">
                  <FaFacebookSquare size="1.25em" className="mr-1" />
                  <span className="text-muted">Partager</span>
                </a>
              </FacebookShareButton>
            </div>
          </div>
        </Card.Body>
      </Card>
    </div>
  );
};

PostPreview.defaultProps = {
  title: 'A post',
  text: dev.loremIpsum,
  username: 'John Coffey',
  previewLength: 200,
  points: 25,
  type: 'info',
  createdOn: '2020-02-29T12:59-0500',
  commentNb: 12,
};

export default PostPreview;
