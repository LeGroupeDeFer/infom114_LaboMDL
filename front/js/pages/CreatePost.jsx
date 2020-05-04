import React, { useEffect, useState } from 'react';
import {
  Form,
  Button,
  Card,
  Container,
  Row,
  Col,
  InputGroup,
  Spinner,
} from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import {
  faBalanceScale,
  faInfo,
  faLightbulb,
  faPenFancy,
  faPlusSquare,
} from '@fortawesome/free-solid-svg-icons';
import { TiDelete } from 'react-icons/ti';
import { faTag } from '@fortawesome/free-solid-svg-icons';
import { useRequest } from '../hooks';
import { useHistory } from 'react-router-dom';
import api from '../lib/api';
import AutoForm from '../components/AutoForm';
import { Option } from '../components/SearchBar';
import { Simple as SimpleError } from '../components/Error';

const types = [
  { value: 'idea', label: Option({ icon: faLightbulb, label: 'Idée' }) },
  { value: 'info', label: Option({ icon: faInfo, label: 'Information' }) },
  { value: 'poll', label: Option({ icon: faBalanceScale, label: 'Sondage' }) },
];

// I didn't find another way to add styles to the select
const primary = '#A0C55F';
const customStyles = {
  control: (base, state) => ({
    ...base,
    boxShadow: state.isFocused ? '0 0 0 1px ' + primary : 0,
    borderColor: state.isFocused ? primary : base.borderColor,
    '&:hover': {
      borderColor: state.isFocused ? primary : primary,
    },
  }),
  option: (styles, { isFocused, isSelected }) => ({
    ...styles,
    color: isSelected ? '' : '',
    backgroundColor: isFocused ? primary : null,
  }),
};

const optionsValidator = (options) =>
  options.reduce((a, option) => a && option.length, true);

function PollOptions() {
  useEffect(() => register('options', ['', ''], false), []); // Must be before useForm!
  const { data, register, onChange } = AutoForm.useForm();

  // If the post is not a poll, this component ought not to be
  if (!data.kind || data.kind.value !== 'poll') return <></>;

  const options = (data.options && data.options.value) || [];

  const addOption = () => onChange('options', [...options, ''], false);
  const popOption = (i) =>
    onChange(
      'options',
      options.filter((_, j) => i !== j),
      optionsValidator(options)
    );
  const updateOption = (i, value) =>
    onChange(
      'options',
      options.map((option, j) => (i === j ? value : option)),
      optionsValidator(options)
    );

  return (
    <Form.Group>
      {options.map((value, i) => (
        <div key={i}>
          <InputGroup className="pb-3">
            <Form.Control
              type="text"
              placeholder={`Option ${i + 1}`}
              value={value}
              onChange={(e) => updateOption(i, e.target.value)}
              isValid={value !== ''}
            />
            {i > 1 && (
              <InputGroup.Append>
                <Button variant="outline-danger" onClick={(_) => popOption(i)}>
                  <TiDelete size={20} />
                </Button>
              </InputGroup.Append>
            )}
          </InputGroup>
        </div>
      ))}

      {options.length < 5 && (
        <a href="#" onClick={addOption}>
          <Icon icon={faPlusSquare} className="mr-1" />
          <span>Ajouter une option</span>
        </a>
      )}
    </Form.Group>
  );
}

function Submit({ loading }) {
  const InnerSubmit = loading ? (
    <Spinner
      as="span"
      animation="border"
      size="sm"
      role="status"
      aria-hidden="true"
    />
  ) : (
    <span>Créer</span>
  );

  return (
    <AutoForm.Submit
      variant="primary"
      className="mt-1 float-right"
      disabled={!loading}
    >
      {InnerSubmit}
    </AutoForm.Submit>
  );
}

function PostWriter() {
  const history = useHistory();
  const [tagError, data] = useRequest(api.tags, []);
  const [postError, setPostError] = useState(null);
  const tags = (data ? data.tags : []).map((tag) => ({
    id: tag.id,
    value: tag.label,
    label: Option({ icon: faTag, label: tag.label }),
  }));
  const [loading, setLoading] = useState(false);
  const [post, setPost] = useState({
    title: '',
    content: '',
    type: '',
    tags: [],
    options: ['', ''],
  });

  function onSubmit(post) {
    setLoading(true);
    setPost(post);

    api.posts
      .add(post)
      .then((newPost) => history.push(`/post/${newPost.id}`))
      .catch((e) => setPostError(e) || setLoading(false));
  }

  return (
    <Container>
      <br />
      <Card>
        <Card.Body>
          <h1 className="mb-4 text-dark">
            <Icon icon={faPenFancy} className="mr-3" />
            <span>Écrire une idée</span>
          </h1>

          <SimpleError error={tagError || postError} className="mb-3" />

          <AutoForm onSubmit={onSubmit}>
            <Row>
              <Col sm={12} md={6} className="pb-3">
                <AutoForm.Select
                  id="kind"
                  name="kind"
                  options={types}
                  styles={customStyles}
                  placeholder="Sélectionner une catégorie"
                />
              </Col>
              <Col sm={12} md={6} className="pb-3">
                <AutoForm.Control
                  variant="primary"
                  id="title"
                  name="title"
                  type="text"
                  placeholder="Titre du post"
                />
              </Col>
            </Row>

            <Row className="pb-3">
              <Col>
                <AutoForm.Select
                  id="tags"
                  name="tags"
                  options={tags}
                  isMulti
                  placeholder={'Sélectionner un ou plusieurs tags'}
                  styles={customStyles}
                />
              </Col>
            </Row>

            <Row className="pb-3">
              <Col>
                <Form.Group>
                  <AutoForm.Control
                    optional
                    id="content"
                    name="content"
                    as="textarea"
                    rows="5"
                    placeholder="Texte..."
                  />
                </Form.Group>
              </Col>
            </Row>

            <PollOptions />

            <Row className="pb-3">
              <Col>
                <Submit loading={loading} />
              </Col>
            </Row>
          </AutoForm>
        </Card.Body>
      </Card>
    </Container>
  );
}

PostWriter.defaultProps = {};

export default PostWriter;
