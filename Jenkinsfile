pipeline {
    stages {
        stage('MegaLinter') {
            agent {
                kubernetes {
                    yaml """
                        kind: Pod
                        metadata:
                          name: jenkins-agent
                        spec:
                          containers:
                          - name: megalinter
                            image: megalinter/megalinter:v5
                            imagePullPolicy: Always
                            tty: true
                            command:
                                - cat
                        """
                }
            }
            steps {
                sh 'export DEFAULT_WORKSPACE=/home/jenkins/agent/workspace/bootloader_feature_jenkins/; /entrypoint.sh'
            }
        }
    }
}
