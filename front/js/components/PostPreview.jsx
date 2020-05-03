import React, { useState, useEffect } from 'react';
import Card from 'react-bootstrap/Card';
import { dev, preview } from '../lib';
import Badge from 'react-bootstrap/Badge';
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
  FaTrashAlt,
  FaLock,
} from 'react-icons/fa';
import clsx from 'clsx';
import DropdownButton from 'react-bootstrap/DropdownButton';
import Dropdown from 'react-bootstrap/Dropdown';
import { FacebookShareButton } from 'react-share';
import { Link } from 'react-router-dom';

const PostPreview = ({
  id,
  title,
  content,
  author,
  score,
  type,
  previewLength,
  createdAt,
  currentFilter,
  comments,
  tags,
  userVote,
  ...otherProps
}) => {
  let vote = '';
  switch (userVote) {
    case -1:
      vote = 'down';
      break;
    case 1:
      vote = 'up';
      break;
    default:
      vote = 'no';
      break;
  }

  const [voted, setVoted] = useState(vote);
  const [scoreState, setScoreState] = useState(score);

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
        onClick={() => otherProps.show_modal(id)}
        id={id}
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
                {author.firstname}
                {'  '}
                {author.lastname}
              </a>{' '}
              -{' '}
              <Moment locale="fr" fromNow>
                {createdAt}
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
              <Dropdown.Item as="button">
                <FaTrashAlt className="mr-2" />
                Supprimer
              </Dropdown.Item>
              <Dropdown.Item as="button">
                <FaLock className="mr-2" />
                Vérouiller
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
                score={scoreState}
                set_score={setScoreState}
                post_id={id}
              />
              <div
                className={`text-center ${clsx(
                  voted !== 'no' && voted + '-voted'
                )}`}
              >
                <b>{scoreState}</b>
              </div>

              <DownVote
                is_logged={otherProps.is_logged}
                voted={voted}
                set_vote={setVoted}
                score={scoreState}
                set_score={setScoreState}
                post_id={id}
              />
            </div>

            <div className="px-3 pb-3 pt-2">
              <div className="mb-1">
                {tags.map((tag, index) => {
                  return (
                    <a
                      href="#"
                      className="mr-2 tag"
                      onClick={(e) => otherProps.tag_click(e)}
                      value={tag}
                      key={index}
                    >
                      <FaTag className="mr-1" />
                      {tag}
                    </a>
                  );
                })}
              </div>

              <Card.Text>
                {preview(content, previewLength)}{' '}
                <Link to={'/post/' + id}>Lire la suite</Link>
              </Card.Text>

              <Link
                to={'/post/' + id}
                className="post-footer-btn mr-2"
                href="#"
              >
                <MdModeComment size="1.25em" className="mr-1" />
                <span className="text-muted">
                  {comments.length}{' '}
                  {comments.length <= 1 ? 'commentaire' : 'commentaires'}
                </span>
              </Link>

              <FacebookShareButton
                url={'https://unanimity.be/post/' + id}
                quote={title + ' - ' + author.firstname + ' ' + author.lastname}
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

export default PostPreview;
