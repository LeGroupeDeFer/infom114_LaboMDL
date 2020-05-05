import React from 'react';
import { Container, Row, Col } from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { faTag, faSearch } from '@fortawesome/free-solid-svg-icons';
import { components } from 'react-select';
import CreatableSelect from 'react-select/creatable';
import {trace} from "unanimity/lib";

const DropdownIndicator = (props) => {
  return (
    <components.DropdownIndicator {...props}>
      <Icon icon={faSearch} />
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

// SearchBar :: None => Component
function SearchBar({ tags, choices, onChange, children }) {
  const options = tags.map(({ label, id }) => ({
    value: label,
    label: <Option icon={faTag} label={label} />,
  }));

  const localOnChange = (options, action) => {
    if (!['select-option', 'remove-value'].includes(action.action))
      return;
    onChange((options || []).map(o => o.value));
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

        <Col md={4}>{children}</Col>
      </Row>
    </Container>
  );
}

SearchBar.Option = Option;


export default SearchBar;
