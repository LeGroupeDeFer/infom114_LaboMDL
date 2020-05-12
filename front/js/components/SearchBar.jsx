import React  from 'react';
import {Container, Row, Col, ButtonGroup, OverlayTrigger, Tooltip, Button} from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { useRouteMatch, useHistory } from 'react-router-dom';
import { components } from 'react-select';
import CreatableSelect from 'react-select/creatable';
import clsx from 'clsx';

import { useStream } from 'unanimity/context/streamContext';
import { last, kinds } from 'unanimity/lib';
import {Flexbox, Loading} from "./index";
import Spinner from "react-bootstrap/Spinner";


// FilterBar :: Object => Component
function KindSection({ routeMatch, history }) {
  const stream = useStream();

  const localOnClick = kind => {
    stream.kind.set(kind);
    if (!routeMatch.isExact)
      history.push(routeMatch.path);
  }

  return (
    <ButtonGroup className="kind-section d-flex justify-content-between">
      {kinds.map((kind) => (
        <OverlayTrigger
          key={kind.key}
          placement="bottom"
          overlay={<Tooltip id={kind.key}>{kind.label}</Tooltip>}
        >
          <Button
            key={kind.key}
            className={clsx(
              'kind-choice',
              stream.kind.value.key === kind.key && 'active'
            )}
            onClick={() => localOnClick(kind)}
          >
            <Icon icon={kind.icon} />
          </Button>
        </OverlayTrigger>
      ))}
    </ButtonGroup>
  );
}


const DropdownIndicator = (props) => {
  return (
    <components.DropdownIndicator {...props}>
      <Icon icon="search" />
    </components.DropdownIndicator>
  );
};

// Option :: Object => Component
export function Option({ icon, label }) {
  return (
    <div className="search-option">
      <span className="search-option-icon">
        <Icon icon={icon} />
      </span>
      <span className="search-option-label">{label}</span>
    </div>
  );
}


const searchVariant = {
  kinds: KindSection
};

// SearchBar :: None => Component
function SearchBar({ variant, pending }) {
  const stream = useStream();
  const routeMatch = useRouteMatch();
  const history = useHistory();

  const options = stream.tags.available.map(({ label, id }) => ({
    value: label,
    label: <Option icon="tag" label={label} />,
  }));

  const LocalVariant = searchVariant[variant];

  const localOnChange = (options, meta) => {
    // The new option is always the last of react-select values
    const option = last(options);

    if (meta.action === 'create-option')
      stream.keywords.add(option.value);
    else if (meta.action === 'select-option')
      stream.tags.add(option.value);
    else if (meta.action === 'remove-value') {
      const v = meta.removedValue.value;
      if (stream.tags.value.includes(v))
        stream.tags.remove(v);
      else
        stream.keywords.remove(v);
    }

    // If we're not at this route root, we redirect to it
    if (!routeMatch.isExact)
      history.push(routeMatch.path);
  }

  return (
    <Container fluid className="search-container py-2">
      <Row>
        <Col md={4}>
          <CreatableSelect
            id="search-bar"
            className="search-bar my-1"
            classNamePrefix="search-bar"
            isMulti
            options={options}
            components={{ DropdownIndicator }}
            placeholder="Rechercher"
            formatCreateLabel={(userInput) => `Rechercher "${userInput}"`}
            onChange={localOnChange}
          />
        </Col>

        <Col md={4}>
          <LocalVariant
            routeMatch={routeMatch}
            history={history}
          />
        </Col>

        <Col md={4} style={{ display: pending ? 'block' : 'none' }}>
          <Flexbox reverse align="center" className="h-100">
            <Spinner
              animation="border"
              variant="primary"
              role="status"
            />
          </Flexbox>
        </Col>
      </Row>
    </Container>
  );
}


SearchBar.Option = Option;


export default SearchBar;
