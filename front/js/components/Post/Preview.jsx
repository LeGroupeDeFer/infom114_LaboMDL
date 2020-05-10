import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import Moment from '../Moment';
import { Badge, Card, Dropdown, DropdownButton } from 'react-bootstrap';
import { DownVote, UpVote } from './Vote';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import clsx from 'clsx';
import { FacebookShareButton } from 'react-share';
import { useAuth } from 'unanimity/context/authContext';

import { preview } from 'unanimity/lib';


function getDisplayedKind(kind) {
  switch (kind) {
    case 'info':
      return 'Information';
    case 'poll':
      return 'Sondage';
    case 'idea':
      return 'Idée';
  }
}

const Preview = ({
  post,
  previewLength,
  currentFilter,
  showPreviewModal,
  showDeleteModal,
  showReportModal,
  onClick,
  onTagClick,
  ...others
}) => {
  const { user, token } = useAuth();
  const isLogged = !!user;
  let caps;
  token != null ? (caps = token.cap) : (caps = []);

  const {
    id,
    title,
    content,
    author,
    score,
    kind,
    createdAt,
    comments,
    tags,
    userVote,
  } = post;

  let vote = ['down', 'no', 'up'][userVote + 1];
  let owner = user == null ? false : author.id == user.id;
  const [voted, setVoted] = useState(vote);
  const [scoreState, setScoreState] = useState(score);

  const deletePost = () => showDeleteModal(id);

  const reportPost = () => showReportModal(id);

  return (
    <div className="d-flex">
      <Card {...others} className="post" onClick={() => onClick(id)} id={id}>
        <Card.Header>
          <h5>
            <Badge className={`post-${kind} mr-2`}>
              {getDisplayedKind(kind)}
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
              <Moment date={createdAt} />
            </span>

            <DropdownButton
              title={
                <span>
                  <Icon icon="ellipsis-h" />
                </span>
              }
              variant="link"
              className="float-right more btn-link"
              onClick={(e) => e.stopPropagation()}
            >
              {caps.some((e) => e.name === 'post:hide') && (
                <Dropdown.Item as="button">
                  <Icon icon="eye-slash" className="mr-2" />
                  Masquer
                </Dropdown.Item>
              )}
              <Dropdown.Item as="button" onClick={reportPost}>
                <Icon icon="flag" className="mr-2" />
                Signaler
              </Dropdown.Item>

              {owner && (
                <Dropdown.Item as="button" onClick={deletePost}>
                  <Icon icon="trash-alt" className="mr-2" />
                  Supprimer
                </Dropdown.Item>
              )}

              {caps.some((e) => e.name === 'post:lock') && (
                <Dropdown.Item as="button">
                  <Icon icon="lock" className="mr-2" />
                  Vérouiller
                </Dropdown.Item>
              )}
            </DropdownButton>
          </h5>
        </Card.Header>

        <Card.Body className="p-0">
          <div className="d-flex">
            <div className="vote-section">
              <UpVote
                isLogged={isLogged}
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
                isLogged={isLogged}
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
                      onClick={(e) => onTagClick(e)}
                      value={tag}
                      key={index}
                    >
                      <Icon icon="tag" className="mr-1" />
                      {tag}
                    </a>
                  );
                })}
              </div>

              <Card.Text>
                {preview(content, previewLength)}{' '}
                <Link to={'/detail/' + id}>Lire la suite</Link>
              </Card.Text>

              <Link
                to={'/detail/' + id}
                className="post-footer-btn mr-2"
                href="#"
              >
                <Icon icon="comment-alt" size="1.25em" className="mr-1" />
                <span className="text-muted">
                  {comments.length}
                  {comments.length <= 1 ? 'commentaire' : 'commentaires'}
                </span>
              </Link>

              <FacebookShareButton
                url={'https://unanimity.be/detail/' + id}
                quote={title + ' - ' + author.firstname + ' ' + author.lastname}
                onClick={(e) => e.stopPropagation()}
              >
                <a className="post-footer-btn mr-2" href="#">
                  <Icon
                    icon={['fab', 'facebook-square']}
                    size="1.25em"
                    className="mr-1"
                  />
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

export default Preview;
