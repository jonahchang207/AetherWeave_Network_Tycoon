using UnityEngine;

namespace AetherWeave.Core
{
    /// <summary>
    /// Entry point for scene load. Wires simulation tick and UI when subsystems exist.
    /// </summary>
    public sealed class GameBootstrap : MonoBehaviour
    {
        [SerializeField] float simulationTickSeconds = 0.25f;

        float _accumulator;

        void Update()
        {
            _accumulator += Time.deltaTime;
            while (_accumulator >= simulationTickSeconds)
            {
                _accumulator -= simulationTickSeconds;
                TickSimulation();
            }
        }

        static void TickSimulation()
        {
            // Phase 1: invoke native flow engine + ECS systems here.
        }
    }
}
