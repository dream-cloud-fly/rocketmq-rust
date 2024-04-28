/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    base::{commit_log_dispatcher::CommitLogDispatcher, dispatch_request::DispatchRequest},
    queue::local_file_consume_queue_store::{LocalFileConsumeQueue, LocalFileConsumeQueueStore},
};

pub struct CommitLogDispatcherBuildConsumeQueue {
    consume_queue_store: Arc<Mutex<LocalFileConsumeQueueStore<LocalFileConsumeQueue>>>,
}

impl CommitLogDispatcherBuildConsumeQueue {
    pub fn new(
        consume_queue_store: Arc<Mutex<LocalFileConsumeQueueStore<LocalFileConsumeQueue>>>,
    ) -> Self {
        Self {
            consume_queue_store,
        }
    }
}

impl CommitLogDispatcher for CommitLogDispatcherBuildConsumeQueue {
    async fn dispatch(&mut self, dispatch_request: &DispatchRequest) {
        self.consume_queue_store
            .lock()
            .await
            .put_message_position_info_wrapper(dispatch_request);
    }
}