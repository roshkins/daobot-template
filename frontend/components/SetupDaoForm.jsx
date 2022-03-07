import React from 'react';
import PropTypes from 'prop-types';

export default function Form({ onSubmit, daoId, setDaoId, currentUser }) {
    return (
        <form onSubmit={onSubmit}>
            <fieldset id="fieldset">
                <p></p>
                <p className="highlight">
                    <label htmlFor="message">DAO account id:</label>
                    <input
                        autoFocus
                        id="accountId"
                        required
                        value={daoId}
                        onChange={(e) => { setDaoId(e.target.value); }}
                    />
                </p>
                <button type="submit">
                    Connect
                </button>
            </fieldset>
        </form>
    );
}

Form.propTypes = {
    onSubmit: PropTypes.func.isRequired,
    currentUser: PropTypes.shape({
        accountId: PropTypes.string.isRequired
    })
};
