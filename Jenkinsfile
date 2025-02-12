pipeline {
    agent any
    options {
        skipStagesAfterUnstable()
    }
    stages {
        stage('Build') {
            steps {
                sh 'make build-docker'
                sh '~/.cargo/bin/cargo build --bin tvl_maker --release'
                sh 'cp target/release/tvl_maker ~/tvl_maker'
            }
        }
    }
}