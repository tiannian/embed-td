use time::Duration;

#[derive(Debug, Clone)]
pub struct ConsensusConfig {
    /// How long we wait for a proposal block before prevoting nil
    pub timeout_propose: Duration,
    /// How much timeout_propose increases with each round
    pub timeout_propose_delta: Duration,
    /// How long we wait after receiving +2/3 prevotes for “anything” (ie. not a single block or nil)
    pub timeout_prevote: Duration,
    /// How much the timeout_prevote increases with each round
    pub timeout_prevote_delta: Duration,
    /// How long we wait after receiving +2/3 precommits for “anything” (ie. not a single block or nil)
    pub timeout_precommit: Duration,
    /// How much the timeout_precommit increases with each round
    pub timeout_precommit_delta: Duration,
    /// How long we wait after committing a block, before starting on the new
    /// height (this gives us a chance to receive some more precommits, even
    /// though we already have +2/3).
    pub timeout_commit: Duration,

    /// How many blocks to look back to check existence of the node's consensus votes before joining consensus
    /// When non-zero, the node will panic upon restart
    /// if the same consensus key was used to sign {double_sign_check_height} last blocks.
    /// So, validators should stop the state machine, wait for some blocks, and then restart the state machine to avoid panic.
    pub double_sign_check_height: u64,

    /// Make progress as soon as we have all the precommits (as if TimeoutCommit = 0)
    pub skip_timeout_commit: bool,

    /// EmptyBlocks mode and possible interval between empty blocks
    pub create_empty_blocks: bool,
    pub create_empty_blocks_interval: Duration,

    /// Reactor sleep duration parameters
    pub peer_gossip_sleep_duration: Duration,
    pub peer_query_maj23_sleep_duration: Duration,

    /// Set to true to discard ABCI responses from the state store, which can save a
    /// considerable amount of disk space. Set to false to ensure ABCI responses are
    /// persisted. ABCI responses are required for /block_results RPC queries, and to
    /// reindex events in the command-line tool.
    pub discard_abci_responses: bool,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            timeout_propose: Duration::seconds(3),
            timeout_propose_delta: Duration::milliseconds(500),
            timeout_prevote: Duration::seconds(1),
            timeout_prevote_delta: Duration::milliseconds(500),
            timeout_precommit: Duration::seconds(1),
            timeout_precommit_delta: Duration::milliseconds(500),
            timeout_commit: Duration::seconds(1),
            double_sign_check_height: 0,
            skip_timeout_commit: false,
            create_empty_blocks: true,
            create_empty_blocks_interval: Duration::seconds(0),
            peer_gossip_sleep_duration: Duration::milliseconds(100),
            peer_query_maj23_sleep_duration: Duration::seconds(2),
            discard_abci_responses: false,
        }
    }
}
